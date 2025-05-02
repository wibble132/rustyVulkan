use crate::result::{err, error, Result};
use glfw::{ClientApiHint, Glfw, PWindow, WindowHint, WindowMode};
use shared::{UniformBufferObject, VertexData};
use std::collections::BTreeSet;
use std::f32::consts::PI;
use std::fmt::Debug;
use std::mem::{offset_of, MaybeUninit};
use std::path::Path;
use std::ptr::null;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock};
use std::time::{Duration, Instant};
use std::{ffi, ptr, slice};
use std::thread::sleep;

const VALIDATION_LAYERS: &[*const ffi::c_char] = &[c"VK_LAYER_KHRONOS_validation".as_ptr()];
const ENABLE_VALIDATION: bool = cfg!(any(debug_assertions, not(debug_assertions)));
static DEVICE_EXTENSIONS: &[&ffi::CStr] = &[ash::vk::KHR_SWAPCHAIN_NAME];
const MAX_FRAMES_IN_FLIGHT: u32 = 2;

type DebugCallback = fn(
    ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    ash::vk::DebugUtilsMessageTypeFlagsEXT,
    ash::vk::DebugUtilsMessengerCallbackDataEXT<'_>,
);

pub(crate) struct VulkanApp {
    glfw: Glfw,
    window: PWindow,

    vulkan: VulkanData,

    current_frame: u32,

    framebuffer_resized: Arc<AtomicBool>,
}

struct VulkanData {
    pub entry: ash::Entry,
    pub instance: ash::Instance,

    pub debug_utils_instance: Option<ash::ext::debug_utils::Instance>,
    pub debug_callback: Option<ash::vk::DebugUtilsMessengerEXT>,

    pub surface_instance: ash::khr::surface::Instance,
    pub surface: ash::vk::SurfaceKHR,

    pub physical_device: ash::vk::PhysicalDevice,
    pub device: ash::Device,
    pub graphics_queue: ash::vk::Queue,
    pub present_queue: ash::vk::Queue,

    pub swapchain_device: ash::khr::swapchain::Device,
    pub swapchain: ash::vk::SwapchainKHR,
    pub swapchain_images: Vec<ash::vk::Image>,
    pub swapchain_format: ash::vk::Format,
    pub swapchain_extent: ash::vk::Extent2D,
    pub swapchain_image_views: Vec<ash::vk::ImageView>,
    pub swap_chain_framebuffers: Vec<ash::vk::Framebuffer>,

    pub shader_module: ash::vk::ShaderModule,
    pub render_pass: ash::vk::RenderPass,
    pub descriptor_set_layout: ash::vk::DescriptorSetLayout,
    pub pipeline_layout: ash::vk::PipelineLayout,
    pub graphics_pipeline: ash::vk::Pipeline,
    pub command_pool: ash::vk::CommandPool,
    pub texture_image: ash::vk::Image,
    pub texture_image_memory: ash::vk::DeviceMemory,
    pub texture_image_view: ash::vk::ImageView,
    pub texture_sampler: ash::vk::Sampler,
    pub vertex_buffer: ash::vk::Buffer,
    pub vertex_buffer_memory: ash::vk::DeviceMemory,
    pub index_buffer: ash::vk::Buffer,
    pub index_buffer_memory: ash::vk::DeviceMemory,
    pub uniform_buffers: Vec<ash::vk::Buffer>,
    pub uniform_buffers_memory: Vec<ash::vk::DeviceMemory>,
    pub uniform_buffers_mapped: Vec<*mut ffi::c_void>,
    pub descriptor_pool: ash::vk::DescriptorPool,
    pub descriptor_sets: Vec<ash::vk::DescriptorSet>,
    pub command_buffers: Vec<ash::vk::CommandBuffer>,

    pub image_available_semaphores: Vec<ash::vk::Semaphore>,
    pub render_finished_semaphores: Vec<ash::vk::Semaphore>,
    pub in_flight_fences: Vec<ash::vk::Fence>,
}

#[rustfmt::skip] // This doesn't need to get shoved onto so many lines
const VERTICES: [VertexData; 4] = [
    VertexData {
        position: glam::Vec2 { x: -0.5, y: -0.5 },
        colour: glam::Vec3 { x: 1.0, y: 0.0, z: 0.0 },
        tex_coord: glam::Vec2 { x: 1.0, y: 0.0 },
    },
    VertexData {
        position: glam::Vec2 { x: 0.5, y: -0.5 },
        colour: glam::Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        tex_coord: glam::Vec2 { x: 0.0, y: 0.0 },
    },
    VertexData {
        position: glam::Vec2 { x: 0.5, y: 0.5 },
        colour: glam::Vec3 { x: 0.0, y: 0.0, z: 1.0 },
        tex_coord: glam::Vec2 { x: 0.0, y: 1.0 },
    },
    VertexData {
        position: glam::Vec2 { x: -0.5, y: 0.5 },
        colour: glam::Vec3 { x: 1.0, y: 1.0, z: 1.0 },
        tex_coord: glam::Vec2 { x: 1.0, y: 1.0 },
    },
];

const INDICES: [u16; 6] = [0, 1, 2, 2, 3, 0];

trait VertexInputData {
    fn get_binding_description() -> ash::vk::VertexInputBindingDescription;
    fn get_attribute_descriptions() -> [ash::vk::VertexInputAttributeDescription; 3];
}

impl VertexInputData for VertexData {
    fn get_binding_description() -> ash::vk::VertexInputBindingDescription {
        ash::vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(size_of::<VertexData>() as u32)
            .input_rate(ash::vk::VertexInputRate::VERTEX)
    }
    fn get_attribute_descriptions() -> [ash::vk::VertexInputAttributeDescription; 3] {
        [
            ash::vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(0)
                .format(ash::vk::Format::R32G32_SFLOAT)
                .offset(offset_of!(VertexData, position) as u32),
            ash::vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(1)
                .format(ash::vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(VertexData, colour) as u32),
            ash::vk::VertexInputAttributeDescription::default()
                .binding(0)
                .location(2)
                .format(ash::vk::Format::R32G32_SFLOAT)
                .offset(offset_of!(VertexData, tex_coord) as u32),
        ]
    }
}

impl VulkanApp {
    pub(crate) fn new() -> Result<Self> {
        // Initialise
        println!("Creating vulkan app");
        let (glfw, window, framebuffer_resized) = Self::init_window()?;
        let vulkan = Self::init_vulkan(&glfw, &window)?;

        Ok(Self {
            glfw,
            window,
            vulkan,
            current_frame: 0,
            framebuffer_resized,
        })
    }
}

impl VulkanApp {
    pub const WIDTH: u32 = 800;
    pub const HEIGHT: u32 = 600;

    pub fn run(&mut self) {
        self.main_loop();
    }

    fn init_window() -> Result<(Glfw, PWindow, Arc<AtomicBool>)> {
        let callback = |x, y| println!("Callback error while loading glfw: {x}, {y}");
        let mut glfw =
            glfw::init(callback).map_err(|e| err(&format!("Failed to initialise glfw: {e:?}")))?;

        // Disable OpenGL since we want to use Vulkan
        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        // Disable resizing until we support recreating the swapchain
        glfw.window_hint(WindowHint::Resizable(true));

        let (mut window, _) = glfw
            .create_window(Self::WIDTH, Self::HEIGHT, "Vulkan", WindowMode::Windowed)
            .ok_or(err("Failed to create a window"))?;

        let framebuffer_resized = Arc::new(AtomicBool::new(false));
        {
            let fb_resize = framebuffer_resized.clone();
            window.set_framebuffer_size_callback(move |_, _, _| {
                fb_resize.store(true, Ordering::Relaxed)
            });
        }

        Ok((glfw, window, framebuffer_resized))
    }
    fn init_vulkan(glfw: &Glfw, window: &glfw::Window) -> Result<VulkanData> {
        // TODO Consider safety arguments of dynamically loading the library, and maybe handle a failure with some nicer logs?
        println!("Loading Vulkan library");
        let entry = unsafe { ash::Entry::load()? };
        // let entry = ash::Entry::linked();
        println!("Loaded Vulkan library");

        if ENABLE_VALIDATION && !Self::check_validation_layer_support(&entry) {
            return error("Validation layers requested, but not available.");
        }

        let instance = Self::create_instance(&entry, glfw)?;
        let (debug_utils_instance, debug_callback) = Self::setup_debug_messenger(&entry, &instance);

        let surface = Self::create_surface(&instance, window)?;

        let surface_instance = ash::khr::surface::Instance::new(&entry, &instance);
        let (physical_device, device_properties) =
            unsafe { Self::pick_physical_device(&instance, &surface_instance, surface)? };
        // Safety: the PhysicalDevice from `pick_physical_device` satisfies `is_device_suitable`
        let (device, graphics_queue, present_queue) = unsafe {
            Self::create_logical_device(&instance, &surface_instance, physical_device, surface)
        }?;

        let swapchain_device = ash::khr::swapchain::Device::new(&instance, &device);
        let (swapchain, swapchain_images, swapchain_format, swapchain_extent) = unsafe {
            Self::create_swap_chain(
                window,
                &instance,
                &surface_instance,
                &swapchain_device,
                physical_device,
                surface,
            )
        }?;

        let swapchain_image_views =
            Self::create_image_views(&device, &swapchain_images, swapchain_format)?;

        let render_pass = Self::create_render_pass(&device, swapchain_format)?;

        let descriptor_set_layout = Self::create_descriptor_set_layout(&device)?;

        let (shader_module, pipeline_layout, graphics_pipeline) =
            Self::create_graphics_pipeline(&device, render_pass, &descriptor_set_layout)?;

        let swap_chain_framebuffers = Self::create_framebuffers(
            &device,
            &swapchain_image_views,
            render_pass,
            swapchain_extent,
        )?;

        let command_pool = Self::create_command_pool(
            &instance,
            &device,
            &surface_instance,
            physical_device,
            surface,
        )?;

        let (texture_image, texture_image_memory) = Self::create_texture_image(
            &instance,
            &device,
            physical_device,
            command_pool,
            graphics_queue,
        )?;

        let texture_image_view = Self::create_texture_image_view(&device, texture_image)?;
        let texture_sampler = Self::create_texture_sampler(&device, &device_properties.limits)?;

        let (vertex_buffer, vertex_buffer_memory) = Self::create_vertex_buffer(
            &instance,
            &device,
            command_pool,
            graphics_queue,
            physical_device,
        )?;

        let (index_buffer, index_buffer_memory) = Self::create_index_buffer(
            &instance,
            &device,
            command_pool,
            graphics_queue,
            physical_device,
        )?;

        let (uniform_buffers, uniform_buffers_memory, uniform_buffers_mapped) =
            Self::create_uniform_buffers(&instance, &device, physical_device)?;

        let descriptor_pool = Self::create_descriptor_pool(&device)?;
        let descriptor_sets = Self::create_descriptor_sets(
            &device,
            descriptor_set_layout,
            descriptor_pool,
            &uniform_buffers,
            texture_image_view,
            texture_sampler,
        )?;

        let command_buffers = Self::create_command_buffers(&device, command_pool)?;

        let (image_available_semaphores, render_finished_semaphores, in_flight_fences) =
            Self::create_sync_objects(&device)?;

        Ok(VulkanData {
            entry,
            instance,
            debug_utils_instance,
            debug_callback,
            surface,
            surface_instance,
            physical_device,
            device,
            graphics_queue,
            present_queue,
            swapchain_device,
            swapchain,
            swapchain_images,
            swapchain_format,
            swapchain_extent,
            swapchain_image_views,
            swap_chain_framebuffers,
            shader_module,
            render_pass,
            descriptor_set_layout,
            pipeline_layout,
            graphics_pipeline,
            command_pool,
            texture_image,
            texture_image_memory,
            texture_image_view,
            texture_sampler,
            vertex_buffer,
            vertex_buffer_memory,
            index_buffer,
            index_buffer_memory,
            uniform_buffers,
            uniform_buffers_memory,
            uniform_buffers_mapped,
            descriptor_pool,
            descriptor_sets,
            command_buffers,
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
        })
    }

    fn main_loop(&mut self) {
        const FRAME_COUNT: u32 = 10000;

        let mut x = 0;
        let mut i = Instant::now();
        let mut times = Vec::with_capacity(FRAME_COUNT as usize);

        while !self.window.should_close() {
            self.glfw.poll_events();
            self.draw_frame().unwrap();

            let elapsed = i.elapsed();
            times.push(elapsed);
            i = Instant::now();
            x += 1;
            if x == FRAME_COUNT {
                println!("This has gone on long enough!");
                break;
            }
            
            sleep(Duration::from_millis(1));
        }

        let mean_frame_time = times.iter().sum::<Duration>() / FRAME_COUNT;
        println!("Average frame time is {}us", mean_frame_time.as_micros());
        let fps = 1.0 / mean_frame_time.as_secs_f64();
        println!("That's {fps}fps");

        let max = times.iter().max().unwrap();
        let min = times.iter().min().unwrap();
        println!("Min: {}us, max: {}us", min.as_micros(), max.as_micros());

        unsafe {
            _ = self.vulkan.device.device_wait_idle();
        }
    }
}

impl VulkanApp {
    fn create_instance(entry: &ash::Entry, glfw: &Glfw) -> Result<ash::Instance> {
        let extensions = Self::get_required_extensions(glfw);
        let extension_names = extensions
            .into_iter()
            .map(|name| {
                ffi::CString::new(name.into_bytes())
                    .expect("Extension names should not contain null bytes")
            })
            .collect::<Vec<_>>();
        let extension_names = extension_names
            .iter()
            .map(|s| s.as_ptr())
            .collect::<Vec<_>>();

        let application_info = ash::vk::ApplicationInfo::default()
            .application_name(c"Hello Triangle")
            .application_version(ash::vk::make_api_version(0, 1, 0, 0))
            .engine_name(c"No engine")
            .engine_version(ash::vk::make_api_version(0, 1, 0, 0))
            .api_version(ash::vk::API_VERSION_1_2);

        let create_info = ash::vk::InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_extension_names(&extension_names);

        // Place this outside the `if` to ensure it doesn't get dropped early
        let mut debug_info = Self::get_create_debug_info();
        let create_info = if ENABLE_VALIDATION {
            create_info
                .enabled_layer_names(VALIDATION_LAYERS)
                .push_next(&mut debug_info)
        } else {
            create_info
        };

        let instance = unsafe { entry.create_instance(&create_info, None)? };

        Ok(instance)
    }
    fn setup_debug_messenger(
        entry: &ash::Entry,
        instance: &ash::Instance,
    ) -> (
        Option<ash::ext::debug_utils::Instance>,
        Option<ash::vk::DebugUtilsMessengerEXT>,
    ) {
        if ENABLE_VALIDATION {
            let debug_utils_instance = ash::ext::debug_utils::Instance::new(entry, instance);
            let debug_messenger = match Self::create_debug_callback(&debug_utils_instance) {
                Ok(x) => Some(x),
                Err(e) => {
                    println!("Failed to create debug utils with error: {e}");
                    None
                }
            };
            (Some(debug_utils_instance), debug_messenger)
        } else {
            (None, None)
        }
    }
    fn get_required_extensions(glfw: &Glfw) -> Vec<String> {
        let mut extensions = glfw.get_required_instance_extensions().unwrap_or_default();

        if ENABLE_VALIDATION {
            extensions.push(
                ash::ext::debug_utils::NAME
                    .to_str()
                    .expect("Extension names are valid UTF-8")
                    .to_string(),
            );
        }

        extensions
    }
    fn check_validation_layer_support(entry: &ash::Entry) -> bool {
        let Ok(layers) = (unsafe { entry.enumerate_instance_layer_properties() }) else {
            return false;
        };

        let res = VALIDATION_LAYERS.iter().all(|&layer_name| {
            layers.iter().any(|layer| unsafe {
                ffi::CStr::from_ptr(layer.layer_name.as_ptr()) == ffi::CStr::from_ptr(layer_name)
            })
        });
        println!("{res}");
        res
    }
    fn get_create_debug_info<'a>() -> ash::vk::DebugUtilsMessengerCreateInfoEXT<'a> {
        let callback: Box<DebugCallback> = Box::new(|message_severity, message_type, data| {
            let message = unsafe { data.message_as_c_str() }
                .and_then(|x| x.to_str().ok())
                .unwrap_or_default();
            println!("{message_severity:?}-{message_type:?}: {message}");
        });

        let info = ash::vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(
                ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | ash::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                    | ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
            )
            .message_type(
                ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                    | ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            )
            .user_data(Box::into_raw(callback) as _)
            .pfn_user_callback(Some(debug_callback));

        info
    }
    fn create_debug_callback(
        debug_utils_instance: &ash::ext::debug_utils::Instance,
    ) -> Result<ash::vk::DebugUtilsMessengerEXT> {
        let create_info = Self::get_create_debug_info();
        let messenger =
            unsafe { debug_utils_instance.create_debug_utils_messenger(&create_info, None)? };
        Ok(messenger)
    }
    fn create_surface(
        instance: &ash::Instance,
        window: &glfw::Window,
    ) -> Result<ash::vk::SurfaceKHR> {
        let mut surface = MaybeUninit::<ash::vk::SurfaceKHR>::uninit();
        let result = window.create_window_surface(instance.handle(), null(), surface.as_mut_ptr());

        match result.result() {
            Ok(()) => Ok(unsafe { surface.assume_init() }),
            Err(e) => Err(e.into()),
        }
    }

    /// # Safety
    /// - `surface` MUST be a valid `VkSurfaceKHR` handle
    /// - `surface` MUST be created, allocated, or retrieved from `instance`
    unsafe fn pick_physical_device(
        instance: &ash::Instance,
        surface_instance: &ash::khr::surface::Instance,
        surface: ash::vk::SurfaceKHR,
    ) -> Result<(ash::vk::PhysicalDevice, ash::vk::PhysicalDeviceProperties)> {
        // Safety:
        // - instance is a valid VkInstance
        // - all devices given to `is_device_suitable` are from `enumerate_physical_devices`, so are valid `VkPhysicalDevice` handles
        let device = unsafe {
            let device_list = instance.enumerate_physical_devices()?;
            device_list
                .into_iter()
                .find(|&x| Self::is_device_suitable(instance, surface_instance, x, surface))
        };

        device
            .map(|d| {
                let x = unsafe { instance.get_physical_device_properties(d) };
                (d, x)
            })
            .ok_or_else(|| err("No suitable device found").into())
    }
    /// # SAFETY
    /// - `device` MUST be a valid `VkPhysicalDevice` handle
    /// - `surface` MUST be a valid `VkSurfaceKHR` handle
    /// - `device` and `surface` MUST be created, allocated, or retrieved from the same `VkInstance` `instance`
    unsafe fn is_device_suitable(
        instance: &ash::Instance,
        surface_instance: &ash::khr::surface::Instance,
        device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
    ) -> bool {
        let device_properties = unsafe { instance.get_physical_device_properties(device) };
        let device_features = unsafe { instance.get_physical_device_features(device) };

        let mut features12 = ash::vk::PhysicalDeviceVulkan12Features::default();
        let mut device_features2 =
            ash::vk::PhysicalDeviceFeatures2::default().push_next(&mut features12);
        unsafe { instance.get_physical_device_features2(device, &mut device_features2) };

        // No features needed for now, left here for future reference to use them though
        let _ = (device_properties, device_features);

        // Safety: The requirements for `find_queue_families` are the same as for this function
        let indices =
            unsafe { Self::find_queue_families(instance, surface_instance, device, surface) };

        let extensions_supported = {
            // Safety:
            // - `physicalDevice` is a valid `VkPhysicalDevice` handle
            // - `pLayerName` is null; `pPropertyCount` and `pProperties` is handled by `ash`
            let available_extensions = unsafe {
                instance
                    .enumerate_device_extension_properties(device)
                    .unwrap()
            };
            DEVICE_EXTENSIONS.iter().all(|&required| {
                available_extensions.iter().any(|available|
                    // Safety: extension name is valid null-terminated utf-8 string
                    required == unsafe { ffi::CStr::from_ptr(available.extension_name.as_ptr()) })
            });

            true
        };

        let swap_chain_adequate = if extensions_supported {
            // Safety: `device` is a valid `VkPhysicalDevice` handle and `surface` is a valid `VkSurfaceKHR` handle
            let swap_chain_support =
                unsafe { Self::query_swap_chain_support(surface_instance, device, surface) };
            swap_chain_support.is_ok_and(|swap_chain_support| {
                !swap_chain_support.formats.is_empty()
                    && !swap_chain_support.present_modes.is_empty()
            })
        } else {
            false
        };

        indices.is_complete()
            && extensions_supported
            && swap_chain_adequate
            && features12.vulkan_memory_model == ash::vk::TRUE // RustGPU shaders seem to need this
            && device_features.sampler_anisotropy == ash::vk::TRUE // Anisotropy used in sampling shaders
    }
    /// # SAFETY
    /// - `device` MUST be a valid `VkPhysicalDevice` handle
    /// - `surface` MUST be a valid `VkSurfaceKHR` handle
    /// - `device` and `surface` MUST be created, allocated, or retrieved from the same `VkInstance`
    unsafe fn find_queue_families(
        instance: &ash::Instance,
        surface_instance: &ash::khr::surface::Instance,
        device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
    ) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices {
            graphics_family: None,
            present_family: None,
        };

        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(device) };
        for (index, queue_family) in queue_families.into_iter().enumerate() {
            let index = index.try_into().expect("vkGetPhysicalDeviceQueueFamilyProperties property pQueueFamilyPropertyCount is a u32, so index should fit into u32");

            if queue_family
                .queue_flags
                .intersects(ash::vk::QueueFlags::GRAPHICS)
            {
                indices.graphics_family = Some(index);
            }

            // Safety:
            // - `index` is less than the count returned by get_physical_device_queue_family_properties
            // - `device` is a valid VkPhysicalDevice handle
            // - `surface` is a valid VkSurfaceKHR handle
            let present_support = unsafe {
                // Unwrap used: only failures are out of memory (host or device) or lost surface
                //  all unrecoverable, so just panic
                surface_instance
                    .get_physical_device_surface_support(device, index, surface)
                    .unwrap()
            };
            if present_support {
                indices.present_family = Some(index);
            }

            if indices.is_complete() {
                break;
            }
        }

        indices
    }
    /// # SAFETY
    /// - `device` MUST be a valid `VkPhysicalDevice` handle
    /// - `surface` MUST be a valid `VkSurfaceKHR` handle
    /// - `device` and `surface` MUST be created, allocated, or retrieved from the same `VkInstance` `instance`
    ///
    /// # Panics
    /// If the device is not suitable (as per `is_device_suitable`), this may panic
    unsafe fn create_logical_device(
        instance: &ash::Instance,
        surface_instance: &ash::khr::surface::Instance,
        physical_device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
    ) -> Result<(ash::Device, ash::vk::Queue, ash::vk::Queue)> {
        // Safety: `physical_device` is a valid VkPhysicalDevice handle
        let indices = unsafe {
            Self::find_queue_families(instance, surface_instance, physical_device, surface)
        };

        // Collect the indices into a set to get all the unique ones
        let unique_queue_families = vec![indices.graphics_family, indices.present_family]
            .into_iter()
            .map(|queue_family| queue_family.expect("Physical device should have queue families"))
            .collect::<BTreeSet<_>>();

        let queue_priority = &[1.0];
        let queue_create_infos = unique_queue_families
            .into_iter()
            .map(|queue_family| {
                ash::vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(queue_family)
                    .queue_priorities(queue_priority)
            })
            .collect::<Vec<_>>();

        let device_features = ash::vk::PhysicalDeviceFeatures::default().sampler_anisotropy(true);

        let extensions = DEVICE_EXTENSIONS
            .iter()
            .map(|x| x.as_ptr())
            .collect::<Vec<_>>();

        let mut x = ash::vk::PhysicalDeviceVulkan12Features::default().vulkan_memory_model(true);
        let create_info = ash::vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&device_features)
            .enabled_extension_names(&extensions)
            .push_next(&mut x);

        let device = unsafe { instance.create_device(physical_device, &create_info, None) }?;

        let graphics_queue =
            unsafe { device.get_device_queue(indices.graphics_family.unwrap(), 0) };
        let present_queue = unsafe { device.get_device_queue(indices.present_family.unwrap(), 0) };

        Ok((device, graphics_queue, present_queue))
    }
    /// # Safety
    /// - `device` must a valid `VkPhysicalDevice` handle
    /// - `surface` must a valid `VkSurfaceKHR` handle
    unsafe fn query_swap_chain_support(
        surface_instance: &ash::khr::surface::Instance,
        device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
    ) -> Result<SwapChainSupportDetails> {
        // Safety: `device` and `surface` are valid handles to `VkPhysicalDevice` and `VkSurfaceKHR` resp.
        let capabilities =
            unsafe { surface_instance.get_physical_device_surface_capabilities(device, surface) }?;
        let formats =
            unsafe { surface_instance.get_physical_device_surface_formats(device, surface) }?;
        let present_modes =
            unsafe { surface_instance.get_physical_device_surface_present_modes(device, surface) }?;

        Ok(SwapChainSupportDetails {
            capabilities,
            formats,
            present_modes,
        })
    }
    fn choose_swap_surface_format(
        available_formats: &[ash::vk::SurfaceFormatKHR],
    ) -> ash::vk::SurfaceFormatKHR {
        for available_format in available_formats {
            if available_format.format == ash::vk::Format::B8G8R8A8_SRGB
                && available_format.color_space == ash::vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return *available_format;
            }
        }

        // Default to just any format
        available_formats[0]
    }
    fn choose_swap_present_mode(
        available_present_modes: &[ash::vk::PresentModeKHR],
    ) -> ash::vk::PresentModeKHR {
        if available_present_modes.contains(&ash::vk::PresentModeKHR::MAILBOX) {
            ash::vk::PresentModeKHR::MAILBOX
        } else {
            ash::vk::PresentModeKHR::FIFO
        }
    }
    fn choose_swap_extent(
        window: &glfw::Window,
        capabilities: &ash::vk::SurfaceCapabilitiesKHR,
    ) -> ash::vk::Extent2D {
        if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            let (width, height) = window.get_framebuffer_size();

            let width = (width as u32).clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            );
            let height = (height as u32).clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            );

            ash::vk::Extent2D { width, height }
        }
    }
    /// # Safety
    /// - `physical_device` must a valid `VkPhysicalDevice` handle
    /// - `surface` must a valid `VkSurfaceKHR` handle
    /// - more conditions: TODO
    unsafe fn create_swap_chain(
        window: &glfw::Window,
        instance: &ash::Instance,
        surface_instance: &ash::khr::surface::Instance,
        swapchain_device: &ash::khr::swapchain::Device,
        physical_device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
    ) -> Result<(
        ash::vk::SwapchainKHR,
        Vec<ash::vk::Image>,
        ash::vk::Format,
        ash::vk::Extent2D,
    )> {
        let swap_chain_support =
            unsafe { Self::query_swap_chain_support(surface_instance, physical_device, surface) }?;

        let surface_format = Self::choose_swap_surface_format(&swap_chain_support.formats);
        let present_mode = Self::choose_swap_present_mode(&swap_chain_support.present_modes);
        let extent = Self::choose_swap_extent(window, &swap_chain_support.capabilities);

        let mut image_count = swap_chain_support.capabilities.min_image_count + 1;
        if swap_chain_support.capabilities.max_image_count > 0
            && swap_chain_support.capabilities.max_image_count < image_count
        {
            image_count = swap_chain_support.capabilities.max_image_count;
        }
        let image_count = image_count;

        let create_info = ash::vk::SwapchainCreateInfoKHR::default()
            .surface(surface)
            .min_image_count(image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(ash::vk::ImageUsageFlags::COLOR_ATTACHMENT);

        let indices = unsafe {
            Self::find_queue_families(instance, surface_instance, physical_device, surface)
        };
        let queue_family_indices = [
            indices.graphics_family.unwrap(),
            indices.present_family.unwrap(),
        ];
        let create_info = if indices.graphics_family != indices.present_family {
            create_info
                .image_sharing_mode(ash::vk::SharingMode::CONCURRENT)
                .queue_family_indices(&queue_family_indices)
        } else {
            create_info
                .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
                .queue_family_indices(&[])
        };

        let create_info = create_info
            .pre_transform(swap_chain_support.capabilities.current_transform)
            .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true)
            .old_swapchain(ash::vk::SwapchainKHR::null());

        // Safety: TODO...
        let swapchain = unsafe { swapchain_device.create_swapchain(&create_info, None) }
            .map_err(|e| err(&format!("Failed to create swapchain: {}", e)))?;

        let images = unsafe { swapchain_device.get_swapchain_images(swapchain) }?;

        Ok((swapchain, images, surface_format.format, extent))
    }
    fn create_image_view(
        device: &ash::Device,
        image: ash::vk::Image,
        format: ash::vk::Format,
    ) -> Result<ash::vk::ImageView> {
        let create_info = ash::vk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(ash::vk::ImageViewType::TYPE_2D)
            .format(format)
            .subresource_range(
                ash::vk::ImageSubresourceRange::default()
                    .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1),
            );

        let image_view = unsafe { device.create_image_view(&create_info, None) }?;
        Ok(image_view)
    }
    fn create_image_views(
        device: &ash::Device,
        swap_chain_images: &[ash::vk::Image],
        swap_chain_image_format: ash::vk::Format,
    ) -> Result<Vec<ash::vk::ImageView>> {
        let image_views: Vec<ash::vk::ImageView> = swap_chain_images
            .iter()
            .flat_map(|image| Self::create_image_view(device, *image, swap_chain_image_format))
            .collect();

        Ok(image_views)
    }
    fn create_render_pass(
        device: &ash::Device,
        swapchain_image_format: ash::vk::Format,
    ) -> Result<ash::vk::RenderPass> {
        let colour_attachment = ash::vk::AttachmentDescription::default()
            .format(swapchain_image_format)
            .samples(ash::vk::SampleCountFlags::TYPE_1)
            .load_op(ash::vk::AttachmentLoadOp::CLEAR)
            .store_op(ash::vk::AttachmentStoreOp::STORE)
            .stencil_load_op(ash::vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(ash::vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(ash::vk::ImageLayout::UNDEFINED)
            .final_layout(ash::vk::ImageLayout::PRESENT_SRC_KHR);

        let attachment_ref = ash::vk::AttachmentReference::default()
            .attachment(0)
            .layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let attachment_refs = [attachment_ref];
        let subpass = ash::vk::SubpassDescription::default()
            .pipeline_bind_point(ash::vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&attachment_refs);

        let dependency = ash::vk::SubpassDependency::default()
            .src_subpass(ash::vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(ash::vk::AccessFlags::empty())
            .dst_stage_mask(ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(ash::vk::AccessFlags::COLOR_ATTACHMENT_WRITE);

        let subpasses = [subpass];
        let attachments = [colour_attachment];
        let dependencies = [dependency];
        let render_pass_info = ash::vk::RenderPassCreateInfo::default()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);

        let render_pass = unsafe { device.create_render_pass(&render_pass_info, None) }?;
        Ok(render_pass)
    }
    fn create_descriptor_set_layout(device: &ash::Device) -> Result<ash::vk::DescriptorSetLayout> {
        let ubo_layout_binding = ash::vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(ash::vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(ash::vk::ShaderStageFlags::VERTEX);

        let sampler_layout_binding = ash::vk::DescriptorSetLayoutBinding::default()
            .binding(1)
            .descriptor_count(1)
            .descriptor_type(ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .stage_flags(ash::vk::ShaderStageFlags::FRAGMENT);

        let bindings = [ubo_layout_binding, sampler_layout_binding];

        let layout_info = ash::vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        let layout = unsafe { device.create_descriptor_set_layout(&layout_info, None) }?;
        Ok(layout)
    }
    fn create_shader_module(device: &ash::Device, code: &[u8]) -> Result<ash::vk::ShaderModule> {
        let create_info = ash::vk::ShaderModuleCreateInfo {
            code_size: code.len(),
            p_code: code.as_ptr() as _,
            ..Default::default()
        };

        let shader_module = unsafe { device.create_shader_module(&create_info, None) }?;
        Ok(shader_module)
    }
    fn create_graphics_pipeline(
        device: &ash::Device,
        render_pass: ash::vk::RenderPass,
        descriptor_set_layout: &ash::vk::DescriptorSetLayout,
    ) -> Result<(
        ash::vk::ShaderModule,
        ash::vk::PipelineLayout,
        ash::vk::Pipeline,
    )> {
        const SHADER: &[u8] = include_bytes!(env!("shaders.spv"));
        let shader_module = Self::create_shader_module(device, SHADER)?;

        let vert_shader_stage_info = ash::vk::PipelineShaderStageCreateInfo::default()
            .stage(ash::vk::ShaderStageFlags::VERTEX)
            .module(shader_module)
            .name(c"main_vs");
        let frag_shader_stage_info = ash::vk::PipelineShaderStageCreateInfo::default()
            .stage(ash::vk::ShaderStageFlags::FRAGMENT)
            .module(shader_module)
            .name(c"main_fs");

        let shader_stages = [vert_shader_stage_info, frag_shader_stage_info];

        let binding_descriptions = &[VertexData::get_binding_description()];
        let attribute_descriptions = VertexData::get_attribute_descriptions();

        let vertex_input_info = ash::vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(binding_descriptions)
            .vertex_attribute_descriptions(&attribute_descriptions);
        let input_assembly = ash::vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(ash::vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let dynamic_states = [
            ash::vk::DynamicState::VIEWPORT,
            ash::vk::DynamicState::SCISSOR,
        ];
        let dynamic_state =
            ash::vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);

        let viewport_state = ash::vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .scissor_count(1);

        let rasterizer = ash::vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(ash::vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(ash::vk::CullModeFlags::BACK)
            .front_face(ash::vk::FrontFace::COUNTER_CLOCKWISE)
            .depth_bias_enable(false);

        let multisampling = ash::vk::PipelineMultisampleStateCreateInfo::default()
            .sample_shading_enable(false)
            .rasterization_samples(ash::vk::SampleCountFlags::TYPE_1);

        let colour_blend_attachment = ash::vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(
                ash::vk::ColorComponentFlags::R
                    | ash::vk::ColorComponentFlags::G
                    | ash::vk::ColorComponentFlags::B,
            )
            .blend_enable(false);

        let colour_blend_attachments = [colour_blend_attachment];
        let colour_blending = ash::vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .attachments(&colour_blend_attachments);

        let pipeline_layout_info = ash::vk::PipelineLayoutCreateInfo::default()
            .set_layouts(slice::from_ref(descriptor_set_layout));
        let pipeline_layout =
            unsafe { device.create_pipeline_layout(&pipeline_layout_info, None) }?;

        let pipeline_info = ash::vk::GraphicsPipelineCreateInfo::default()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .color_blend_state(&colour_blending)
            .dynamic_state(&dynamic_state)
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0);

        let infos = &[pipeline_info];
        let pipeline = unsafe {
            device.create_graphics_pipelines(ash::vk::PipelineCache::null(), infos, None)
        }
        .map_err(|(_, e)| e)?;

        Ok((shader_module, pipeline_layout, pipeline[0]))
    }
    fn create_framebuffers(
        device: &ash::Device,
        swap_chain_image_views: &[ash::vk::ImageView],
        render_pass: ash::vk::RenderPass,
        swap_chain_extent: ash::vk::Extent2D,
    ) -> Result<Vec<ash::vk::Framebuffer>> {
        let mut swap_chain_framebuffers = Vec::with_capacity(swap_chain_image_views.len());

        for image_view in swap_chain_image_views {
            let attachments = &[*image_view];
            let framebuffer_info = ash::vk::FramebufferCreateInfo::default()
                .render_pass(render_pass)
                .attachments(attachments)
                .width(swap_chain_extent.width)
                .height(swap_chain_extent.height)
                .layers(1);

            let fb = unsafe { device.create_framebuffer(&framebuffer_info, None) }?;
            swap_chain_framebuffers.push(fb);
        }

        Ok(swap_chain_framebuffers)
    }
    fn create_command_pool(
        instance: &ash::Instance,
        device: &ash::Device,
        surface_instance: &ash::khr::surface::Instance,
        physical_device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
    ) -> Result<ash::vk::CommandPool> {
        let queue_family_indices = unsafe {
            Self::find_queue_families(instance, surface_instance, physical_device, surface)
        };

        let pool_info = ash::vk::CommandPoolCreateInfo::default()
            .flags(ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family_indices.graphics_family.unwrap());

        let command_pool = unsafe { device.create_command_pool(&pool_info, None)? };
        Ok(command_pool)
    }
    fn create_buffer(
        instance: &ash::Instance,
        device: &ash::Device,
        size: ash::vk::DeviceSize,
        physical_device: ash::vk::PhysicalDevice,
        usage: ash::vk::BufferUsageFlags,
        properties: ash::vk::MemoryPropertyFlags,
    ) -> Result<(ash::vk::Buffer, ash::vk::DeviceMemory)> {
        let buffer_info = ash::vk::BufferCreateInfo::default()
            .size(size)
            .usage(usage)
            .sharing_mode(ash::vk::SharingMode::EXCLUSIVE);

        let buffer = unsafe { device.create_buffer(&buffer_info, None) }?;
        let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
        let memory_type_index = Self::find_memory_type(
            instance,
            physical_device,
            memory_requirements.memory_type_bits,
            properties,
        )?;
        let alloc_info = ash::vk::MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index);

        let buffer_memory = unsafe { device.allocate_memory(&alloc_info, None) }?;

        unsafe { device.bind_buffer_memory(buffer, buffer_memory, 0) }?;

        Ok((buffer, buffer_memory))
    }
    fn begin_single_time_commands(
        device: &ash::Device,
        command_pool: ash::vk::CommandPool,
    ) -> Result<ash::vk::CommandBuffer> {
        let alloc_info = ash::vk::CommandBufferAllocateInfo::default()
            .level(ash::vk::CommandBufferLevel::PRIMARY)
            .command_pool(command_pool)
            .command_buffer_count(1);

        let command_buffer = unsafe { device.allocate_command_buffers(&alloc_info) }?
            .into_iter()
            .next()
            .expect("allocate_info.command_buffer_count is 1");

        let begin_info = ash::vk::CommandBufferBeginInfo::default()
            .flags(ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            device.begin_command_buffer(command_buffer, &begin_info)?;
        }

        Ok(command_buffer)
    }
    fn end_single_time_commands(
        device: &ash::Device,
        command_pool: ash::vk::CommandPool,
        graphics_queue: ash::vk::Queue,
        command_buffer: ash::vk::CommandBuffer,
    ) -> Result<()> {
        unsafe {
            device.end_command_buffer(command_buffer)?;

            let submit_info =
                ash::vk::SubmitInfo::default().command_buffers(slice::from_ref(&command_buffer));
            device.queue_submit(
                graphics_queue,
                slice::from_ref(&submit_info),
                ash::vk::Fence::null(),
            )?;
            device.queue_wait_idle(graphics_queue)?;
            device.free_command_buffers(command_pool, slice::from_ref(&command_buffer));
        }

        Ok(())
    }
    fn copy_buffer(
        device: &ash::Device,
        command_pool: ash::vk::CommandPool,
        graphics_queue: ash::vk::Queue,
        src: ash::vk::Buffer,
        dst: ash::vk::Buffer,
        size: ash::vk::DeviceSize,
    ) -> Result<()> {
        let command_buffer = Self::begin_single_time_commands(device, command_pool)?;

        let copy_region = ash::vk::BufferCopy::default()
            .src_offset(0)
            .dst_offset(0)
            .size(size);
        unsafe {
            device.cmd_copy_buffer(command_buffer, src, dst, slice::from_ref(&copy_region));
        }

        Self::end_single_time_commands(device, command_pool, graphics_queue, command_buffer)?;

        Ok(())
    }
    fn transition_image_layout(
        device: &ash::Device,
        command_pool: ash::vk::CommandPool,
        graphics_queue: ash::vk::Queue,
        image: ash::vk::Image,
        old_layout: ash::vk::ImageLayout,
        new_layout: ash::vk::ImageLayout,
    ) -> Result<()> {
        let command_buffer = Self::begin_single_time_commands(device, command_pool)?;

        let barrier = ash::vk::ImageMemoryBarrier::default()
            .old_layout(old_layout)
            .new_layout(new_layout)
            .src_queue_family_index(ash::vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(ash::vk::QUEUE_FAMILY_IGNORED)
            .image(image)
            .subresource_range(
                ash::vk::ImageSubresourceRange::default()
                    .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1),
            )
            .src_access_mask(ash::vk::AccessFlags::empty()) // TODO
            .dst_access_mask(ash::vk::AccessFlags::empty()) // TODO 
            ;

        let (barrier, source_stage, destination_stage) = if old_layout
            == ash::vk::ImageLayout::UNDEFINED
            && new_layout == ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL
        {
            (
                barrier
                    .src_access_mask(ash::vk::AccessFlags::empty())
                    .dst_access_mask(ash::vk::AccessFlags::TRANSFER_WRITE),
                ash::vk::PipelineStageFlags::TOP_OF_PIPE,
                ash::vk::PipelineStageFlags::TRANSFER,
            )
        } else if old_layout == ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL
            && new_layout == ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
        {
            (
                barrier,
                ash::vk::PipelineStageFlags::TRANSFER,
                ash::vk::PipelineStageFlags::FRAGMENT_SHADER,
            )
        } else {
            return error("Unsupported layout transition");
        };

        unsafe {
            device.cmd_pipeline_barrier(
                command_buffer,
                source_stage,
                destination_stage,
                ash::vk::DependencyFlags::default(),
                &[],
                &[],
                slice::from_ref(&barrier),
            )
        };

        Self::end_single_time_commands(device, command_pool, graphics_queue, command_buffer)?;
        Ok(())
    }
    fn copy_buffer_to_image(
        device: &ash::Device,
        command_pool: ash::vk::CommandPool,
        graphics_queue: ash::vk::Queue,
        buffer: ash::vk::Buffer,
        image: ash::vk::Image,
        width: u32,
        height: u32,
    ) -> Result<()> {
        let command_buffer = Self::begin_single_time_commands(device, command_pool)?;

        let region = ash::vk::BufferImageCopy::default()
            .buffer_offset(0)
            .buffer_row_length(0)
            .buffer_image_height(0)
            .image_subresource(
                ash::vk::ImageSubresourceLayers::default()
                    .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                    .mip_level(0)
                    .base_array_layer(0)
                    .layer_count(1),
            )
            .image_offset(ash::vk::Offset3D { x: 0, y: 0, z: 0 })
            .image_extent(ash::vk::Extent3D {
                width,
                height,
                depth: 1,
            });

        unsafe {
            device.cmd_copy_buffer_to_image(
                command_buffer,
                buffer,
                image,
                ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                slice::from_ref(&region),
            )
        };

        Self::end_single_time_commands(device, command_pool, graphics_queue, command_buffer)?;

        Ok(())
    }
    fn create_vertex_buffer(
        instance: &ash::Instance,
        device: &ash::Device,
        command_pool: ash::vk::CommandPool,
        graphics_queue: ash::vk::Queue,
        physical_device: ash::vk::PhysicalDevice,
    ) -> Result<(ash::vk::Buffer, ash::vk::DeviceMemory)> {
        let buffer_size = size_of_val(&VERTICES) as ash::vk::DeviceSize;

        let (staging_buffer, staging_buffer_memory) = Self::create_buffer(
            instance,
            device,
            buffer_size,
            physical_device,
            ash::vk::BufferUsageFlags::TRANSFER_SRC,
            ash::vk::MemoryPropertyFlags::HOST_VISIBLE
                | ash::vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        unsafe {
            let data = device.map_memory(
                staging_buffer_memory,
                0,
                buffer_size,
                ash::vk::MemoryMapFlags::empty(),
            )?;
            // Alignment turns out to be ok, but left this as-is just to be safe
            //  - data will have alignment of `VkPhysicalDeviceLimits::minMemoryMapAlignment` (= 4096 on my system)
            //  - VERTICES has alignment 4, which is a factor of 4096, thus will be fine
            core::ptr::write_unaligned(data as _, VERTICES);
            device.unmap_memory(staging_buffer_memory);
        };

        let (vertex_buffer, vertex_buffer_memory) = Self::create_buffer(
            instance,
            device,
            buffer_size,
            physical_device,
            ash::vk::BufferUsageFlags::TRANSFER_DST | ash::vk::BufferUsageFlags::VERTEX_BUFFER,
            ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        Self::copy_buffer(
            device,
            command_pool,
            graphics_queue,
            staging_buffer,
            vertex_buffer,
            buffer_size,
        )?;

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        }

        Ok((vertex_buffer, vertex_buffer_memory))
    }
    fn create_index_buffer(
        instance: &ash::Instance,
        device: &ash::Device,
        command_pool: ash::vk::CommandPool,
        graphics_queue: ash::vk::Queue,
        physical_device: ash::vk::PhysicalDevice,
    ) -> Result<(ash::vk::Buffer, ash::vk::DeviceMemory)> {
        let buffer_size = size_of_val(&INDICES) as ash::vk::DeviceSize;

        let (staging_buffer, staging_buffer_memory) = Self::create_buffer(
            instance,
            device,
            buffer_size,
            physical_device,
            ash::vk::BufferUsageFlags::TRANSFER_SRC,
            ash::vk::MemoryPropertyFlags::HOST_VISIBLE
                | ash::vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        unsafe {
            let data = device.map_memory(
                staging_buffer_memory,
                0,
                buffer_size,
                ash::vk::MemoryMapFlags::empty(),
            )?;
            core::ptr::write_unaligned(data as _, INDICES);
            device.unmap_memory(staging_buffer_memory);
        };

        let (index_buffer, index_buffer_memory) = Self::create_buffer(
            instance,
            device,
            buffer_size,
            physical_device,
            ash::vk::BufferUsageFlags::TRANSFER_DST | ash::vk::BufferUsageFlags::INDEX_BUFFER,
            ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        Self::copy_buffer(
            device,
            command_pool,
            graphics_queue,
            staging_buffer,
            index_buffer,
            buffer_size,
        )?;

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        }

        Ok((index_buffer, index_buffer_memory))
    }
    fn create_uniform_buffers(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: ash::vk::PhysicalDevice,
    ) -> Result<(
        Vec<ash::vk::Buffer>,
        Vec<ash::vk::DeviceMemory>,
        Vec<*mut ffi::c_void>,
    )> {
        let buffer_size = size_of::<UniformBufferObject>();

        let mut uniform_buffers = Vec::with_capacity(buffer_size);
        let mut uniform_buffers_memory = Vec::with_capacity(buffer_size);
        let mut uniform_buffers_mapped = Vec::with_capacity(buffer_size);

        let buffer_size = buffer_size as ash::vk::DeviceSize;

        for _ in 0..buffer_size {
            let (buffer, memory) = Self::create_buffer(
                instance,
                device,
                buffer_size,
                physical_device,
                ash::vk::BufferUsageFlags::UNIFORM_BUFFER,
                ash::vk::MemoryPropertyFlags::HOST_COHERENT
                    | ash::vk::MemoryPropertyFlags::HOST_VISIBLE,
            )?;

            uniform_buffers.push(buffer);
            uniform_buffers_memory.push(memory);

            let map = unsafe {
                device.map_memory(memory, 0, buffer_size, ash::vk::MemoryMapFlags::empty())
            }?;
            uniform_buffers_mapped.push(map);
        }

        Ok((
            uniform_buffers,
            uniform_buffers_memory,
            uniform_buffers_mapped,
        ))
    }
    fn create_descriptor_pool(device: &ash::Device) -> Result<ash::vk::DescriptorPool> {
        let pool_sizes = [
            ash::vk::DescriptorPoolSize::default()
                .ty(ash::vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(MAX_FRAMES_IN_FLIGHT),
            ash::vk::DescriptorPoolSize::default()
                .ty(ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(MAX_FRAMES_IN_FLIGHT),
        ];
        let pool_info = ash::vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(MAX_FRAMES_IN_FLIGHT);

        let descriptor_pool = unsafe { device.create_descriptor_pool(&pool_info, None) }?;

        Ok(descriptor_pool)
    }
    fn create_descriptor_sets(
        device: &ash::Device,
        descriptor_set_layout: ash::vk::DescriptorSetLayout,
        descriptor_pool: ash::vk::DescriptorPool,
        uniform_buffers: &[ash::vk::Buffer],
        texture_image_view: ash::vk::ImageView,
        texture_sampler: ash::vk::Sampler,
    ) -> Result<Vec<ash::vk::DescriptorSet>> {
        let layouts = [descriptor_set_layout; MAX_FRAMES_IN_FLIGHT as usize];
        let alloc_info = ash::vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool)
            .set_layouts(&layouts);

        let descriptor_sets = unsafe { device.allocate_descriptor_sets(&alloc_info) }?;

        for (&buffer, &descriptor_set) in uniform_buffers.iter().zip(&descriptor_sets) {
            let buffer_info = ash::vk::DescriptorBufferInfo::default()
                .buffer(buffer)
                .offset(0)
                .range(size_of::<UniformBufferObject>() as _);

            let image_info = ash::vk::DescriptorImageInfo::default()
                .image_layout(ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .image_view(texture_image_view)
                .sampler(texture_sampler);

            let descriptor_writes = [
                ash::vk::WriteDescriptorSet::default()
                    .dst_set(descriptor_set)
                    .dst_binding(0)
                    .dst_array_element(0)
                    .descriptor_type(ash::vk::DescriptorType::UNIFORM_BUFFER)
                    .buffer_info(slice::from_ref(&buffer_info)),
                ash::vk::WriteDescriptorSet::default()
                    .dst_set(descriptor_set)
                    .dst_binding(1)
                    .dst_array_element(0)
                    .descriptor_type(ash::vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                    .descriptor_count(1)
                    .image_info(slice::from_ref(&image_info)),
            ];
            unsafe { device.update_descriptor_sets(&descriptor_writes, &[]) }
        }

        Ok(descriptor_sets)
    }
    fn create_command_buffers(
        device: &ash::Device,
        command_pool: ash::vk::CommandPool,
    ) -> Result<Vec<ash::vk::CommandBuffer>> {
        let alloc_info = ash::vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(ash::vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(MAX_FRAMES_IN_FLIGHT);

        let command_buffers = unsafe { device.allocate_command_buffers(&alloc_info) }?;
        Ok(command_buffers)
    }
    fn create_sync_objects(
        device: &ash::Device,
    ) -> Result<(
        Vec<ash::vk::Semaphore>,
        Vec<ash::vk::Semaphore>,
        Vec<ash::vk::Fence>,
    )> {
        let semaphore_info = ash::vk::SemaphoreCreateInfo::default();
        let fence_info =
            ash::vk::FenceCreateInfo::default().flags(ash::vk::FenceCreateFlags::SIGNALED);

        let mut image_available_semaphores = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT as usize);
        let mut render_finished_semaphores = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT as usize);
        let mut in_flight_fences = Vec::with_capacity(MAX_FRAMES_IN_FLIGHT as usize);

        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            image_available_semaphores
                .push(unsafe { device.create_semaphore(&semaphore_info, None) }?);
            render_finished_semaphores
                .push(unsafe { device.create_semaphore(&semaphore_info, None) }?);
            in_flight_fences.push(unsafe { device.create_fence(&fence_info, None) }?);
        }

        Ok((
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
        ))
    }
    fn record_command_buffer(
        &self,
        command_buffer: ash::vk::CommandBuffer,
        image_index: u32,
    ) -> Result<()> {
        let begin_info = ash::vk::CommandBufferBeginInfo::default();
        unsafe {
            self.vulkan
                .device
                .begin_command_buffer(command_buffer, &begin_info)
        }?;

        let clear_values = [ash::vk::ClearValue {
            color: ash::vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        }];

        let render_pass_info = ash::vk::RenderPassBeginInfo::default()
            .render_pass(self.vulkan.render_pass)
            .framebuffer(self.vulkan.swap_chain_framebuffers[image_index as usize])
            .render_area(ash::vk::Rect2D {
                offset: ash::vk::Offset2D { x: 0, y: 0 },
                extent: self.vulkan.swapchain_extent,
            })
            .clear_values(&clear_values);

        unsafe {
            self.vulkan.device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_info,
                ash::vk::SubpassContents::INLINE,
            )
        };

        // Group recording the commands in one unsafe for now
        unsafe {
            self.vulkan.device.cmd_bind_pipeline(
                command_buffer,
                ash::vk::PipelineBindPoint::GRAPHICS,
                self.vulkan.graphics_pipeline,
            );

            let vertex_buffers = [self.vulkan.vertex_buffer];
            let offsets = [0 as ash::vk::DeviceSize];
            self.vulkan.device.cmd_bind_vertex_buffers(
                command_buffer,
                0,
                &vertex_buffers,
                &offsets,
            );
            self.vulkan.device.cmd_bind_index_buffer(
                command_buffer,
                self.vulkan.index_buffer,
                0,
                ash::vk::IndexType::UINT16,
            );

            let viewport = ash::vk::Viewport::default()
                .x(0.0)
                .y(0.0)
                .width(self.vulkan.swapchain_extent.width as f32)
                .height(self.vulkan.swapchain_extent.height as f32)
                .min_depth(0.0)
                .max_depth(1.0);
            self.vulkan
                .device
                .cmd_set_viewport(command_buffer, 0, &[viewport]);

            let scissor = ash::vk::Rect2D::default()
                .offset(ash::vk::Offset2D { x: 0, y: 0 })
                .extent(self.vulkan.swapchain_extent);
            self.vulkan
                .device
                .cmd_set_scissor(command_buffer, 0, &[scissor]);

            self.vulkan.device.cmd_bind_descriptor_sets(
                command_buffer,
                ash::vk::PipelineBindPoint::GRAPHICS,
                self.vulkan.pipeline_layout,
                0,
                slice::from_ref(&self.vulkan.descriptor_sets[self.current_frame as usize]),
                &[],
            );

            self.vulkan
                .device
                .cmd_draw_indexed(command_buffer, INDICES.len() as u32, 1, 0, 0, 0);

            self.vulkan.device.cmd_end_render_pass(command_buffer);
        }

        unsafe { self.vulkan.device.end_command_buffer(command_buffer) }?;

        Ok(())
    }
    fn draw_frame(&mut self) -> Result<()> {
        let current_frame = self.current_frame as usize;

        unsafe {
            self.vulkan.device.wait_for_fences(
                &[self.vulkan.in_flight_fences[current_frame]],
                true,
                u64::MAX,
            )?;
        }

        let acquire_image_result = unsafe {
            self.vulkan.swapchain_device.acquire_next_image(
                self.vulkan.swapchain,
                u64::MAX,
                self.vulkan.image_available_semaphores[current_frame],
                ash::vk::Fence::null(),
            )
        };

        let image_index = match acquire_image_result {
            Err(ash::vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                self.recreate_swap_chain()?;
                return Ok(());
            }
            Ok((image_index, _)) => image_index,
            Err(e) => return Err(e.into()),
        };

        self.update_uniform_buffer(image_index);

        unsafe {
            // Only reset fences if we are submitting work
            self.vulkan
                .device
                .reset_fences(&[self.vulkan.in_flight_fences[current_frame]])?;
        }

        unsafe {
            self.vulkan.device.reset_command_buffer(
                self.vulkan.command_buffers[current_frame],
                ash::vk::CommandBufferResetFlags::default(),
            )?;
            self.record_command_buffer(self.vulkan.command_buffers[current_frame], image_index)?;
        }

        let wait_semaphores = [self.vulkan.image_available_semaphores[current_frame]];
        let wait_stages = [ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [self.vulkan.command_buffers[current_frame]];
        let signal_semaphores = [self.vulkan.render_finished_semaphores[current_frame]];
        let submit_info = ash::vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);
        let submit_info = [submit_info];

        unsafe {
            self.vulkan.device.queue_submit(
                self.vulkan.graphics_queue,
                &submit_info,
                self.vulkan.in_flight_fences[current_frame],
            )
        }?;

        let swapchains = [self.vulkan.swapchain];
        let image_indices = [image_index];
        let present_info = ash::vk::PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        let present_result = unsafe {
            self.vulkan
                .swapchain_device
                .queue_present(self.vulkan.present_queue, &present_info)
        };
        let resized = self.framebuffer_resized.load(Ordering::Relaxed);
        match (present_result, resized) {
            (Ok(true), _) | (_, true) => {
                // Suboptimal or resized
                self.framebuffer_resized.store(false, Ordering::Relaxed);
                self.recreate_swap_chain()?;
            }
            (Err(e), _) => return Err(e.into()),
            (Ok(false), false) => { /* All good */ }
        }

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;

        Ok(())
    }
    fn update_uniform_buffer(&mut self, current_image: u32) {
        static START_TIME: LazyLock<Instant> = LazyLock::new(Instant::now);
        LazyLock::force(&START_TIME);

        let current_time = Instant::now();
        let time = (current_time - *START_TIME).as_secs_f32();

        let model = glam::Mat4::from_axis_angle(glam::Vec3::Z, time * PI / 2.0);
        let view = glam::Mat4::look_at_rh(
            glam::Vec3::splat(2.0),
            glam::Vec3::splat(0.0),
            glam::Vec3::Z,
        );
        let mut projection = glam::Mat4::perspective_rh(
            PI / 4.0,
            self.vulkan.swapchain_extent.width as f32 / self.vulkan.swapchain_extent.height as f32,
            0.1,
            10.0,
        );
        projection.y_axis.y *= -1.0;

        let ubo = UniformBufferObject {
            model,
            view,
            projection,
        };

        let map = self.vulkan.uniform_buffers_mapped[current_image as usize];
        unsafe { ptr::write_unaligned(map as _, ubo) };
    }
    fn recreate_swap_chain(&mut self) -> Result<()> {
        let (mut width, mut height) = self.window.get_framebuffer_size();
        while width == 0 && height == 0 {
            (width, height) = self.window.get_framebuffer_size();
            self.glfw.wait_events()
        }

        unsafe { self.vulkan.device.device_wait_idle() }?;

        unsafe {
            VulkanData::cleanup_swapchain(
                &self.vulkan.device,
                &self.vulkan.swapchain_device,
                std::mem::take(&mut self.vulkan.swap_chain_framebuffers),
                std::mem::take(&mut self.vulkan.swapchain_image_views),
                self.vulkan.swapchain,
            );
        }

        (
            self.vulkan.swapchain,
            self.vulkan.swapchain_images,
            self.vulkan.swapchain_format,
            self.vulkan.swapchain_extent,
        ) = unsafe {
            Self::create_swap_chain(
                &self.window,
                &self.vulkan.instance,
                &self.vulkan.surface_instance,
                &self.vulkan.swapchain_device,
                self.vulkan.physical_device,
                self.vulkan.surface,
            )
        }?;

        self.vulkan.swapchain_image_views = Self::create_image_views(
            &self.vulkan.device,
            &self.vulkan.swapchain_images,
            self.vulkan.swapchain_format,
        )?;

        self.vulkan.swap_chain_framebuffers = Self::create_framebuffers(
            &self.vulkan.device,
            &self.vulkan.swapchain_image_views,
            self.vulkan.render_pass,
            self.vulkan.swapchain_extent,
        )?;

        Ok(())
    }
    fn find_memory_type(
        instance: &ash::Instance,
        physical_device: ash::vk::PhysicalDevice,
        type_filter: u32,
        properties: ash::vk::MemoryPropertyFlags,
    ) -> Result<u32> {
        let mem_properties =
            unsafe { instance.get_physical_device_memory_properties(physical_device) };

        for (i, &memory_type) in mem_properties.memory_types_as_slice().iter().enumerate() {
            if type_filter & (1 << i) != 0 && memory_type.property_flags.contains(properties) {
                return Ok(i as u32);
            }
        }

        Err(err("Failed to find memory type").into())
    }
    fn create_texture_image(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: ash::vk::PhysicalDevice,
        command_pool: ash::vk::CommandPool,
        graphics_queue: ash::vk::Queue,
    ) -> Result<(ash::vk::Image, ash::vk::DeviceMemory)> {
        let path = Path::new("res/texture.png");
        let image = image::open(path)
            .expect("Failed to load texture.png")
            .to_rgba8();
        let height = image.height();
        let width = image.width();
        let pixels: &[_] = &*image;

        let (staging_buffer, staging_buffer_memory) = Self::create_buffer(
            instance,
            device,
            pixels.len() as _,
            physical_device,
            ash::vk::BufferUsageFlags::TRANSFER_SRC,
            ash::vk::MemoryPropertyFlags::HOST_VISIBLE
                | ash::vk::MemoryPropertyFlags::HOST_COHERENT,
        )?;

        unsafe {
            let data = device.map_memory(
                staging_buffer_memory,
                0,
                pixels.len() as _,
                ash::vk::MemoryMapFlags::empty(),
            )?;
            ptr::copy_nonoverlapping(pixels.as_ptr(), data as _, pixels.len());
            device.unmap_memory(staging_buffer_memory);
        };

        let (image, memory) = Self::create_image(
            instance,
            device,
            physical_device,
            width,
            height,
            ash::vk::Format::R8G8B8A8_SRGB,
            ash::vk::ImageTiling::OPTIMAL,
            ash::vk::ImageUsageFlags::TRANSFER_DST | ash::vk::ImageUsageFlags::SAMPLED,
            ash::vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;

        Self::transition_image_layout(
            device,
            command_pool,
            graphics_queue,
            image,
            ash::vk::Format::R8G8B8A8_SRGB,
            ash::vk::ImageLayout::UNDEFINED,
            ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        )?;

        Self::copy_buffer_to_image(
            device,
            command_pool,
            graphics_queue,
            staging_buffer,
            image,
            width,
            height,
        )?;

        Self::transition_image_layout(
            device,
            command_pool,
            graphics_queue,
            image,
            ash::vk::Format::R8G8B8A8_SRGB,
            ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        )?;

        unsafe {
            device.destroy_buffer(staging_buffer, None);
            device.free_memory(staging_buffer_memory, None);
        }

        Ok((image, memory))
    }
    fn create_texture_image_view(
        device: &ash::Device,
        texture_image: ash::vk::Image,
    ) -> Result<ash::vk::ImageView> {
        Self::create_image_view(device, texture_image, ash::vk::Format::R8G8B8A8_SRGB)
    }
    fn create_texture_sampler(
        device: &ash::Device,
        device_limits: &ash::vk::PhysicalDeviceLimits,
    ) -> Result<ash::vk::Sampler> {
        let sampler_info = ash::vk::SamplerCreateInfo::default()
            .mag_filter(ash::vk::Filter::LINEAR)
            .min_filter(ash::vk::Filter::LINEAR)
            .address_mode_u(ash::vk::SamplerAddressMode::REPEAT)
            .address_mode_v(ash::vk::SamplerAddressMode::REPEAT)
            .address_mode_w(ash::vk::SamplerAddressMode::REPEAT)
            .anisotropy_enable(true)
            .max_anisotropy(device_limits.max_sampler_anisotropy)
            .border_color(ash::vk::BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(ash::vk::CompareOp::ALWAYS)
            .mipmap_mode(ash::vk::SamplerMipmapMode::LINEAR)
            .mip_lod_bias(0.0)
            .min_lod(0.0)
            .max_lod(0.0);

        let sampler = unsafe { device.create_sampler(&sampler_info, None)? };
        Ok(sampler)
    }
    fn create_image(
        instance: &ash::Instance,
        device: &ash::Device,
        physical_device: ash::vk::PhysicalDevice,
        width: u32,
        height: u32,
        format: ash::vk::Format,
        tiling: ash::vk::ImageTiling,
        usage: ash::vk::ImageUsageFlags,
        properties: ash::vk::MemoryPropertyFlags,
    ) -> Result<(ash::vk::Image, ash::vk::DeviceMemory)> {
        let image_info = ash::vk::ImageCreateInfo::default()
            .image_type(ash::vk::ImageType::TYPE_2D)
            .extent(ash::vk::Extent3D {
                width,
                height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .format(format)
            .tiling(tiling)
            .initial_layout(ash::vk::ImageLayout::UNDEFINED)
            .usage(usage)
            .sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
            .samples(ash::vk::SampleCountFlags::TYPE_1)
            .flags(ash::vk::ImageCreateFlags::empty());

        let image = unsafe { device.create_image(&image_info, None)? };

        let mem_requirements = unsafe { device.get_image_memory_requirements(image) };

        let alloc_info = ash::vk::MemoryAllocateInfo::default()
            .allocation_size(mem_requirements.size)
            .memory_type_index(Self::find_memory_type(
                instance,
                physical_device,
                mem_requirements.memory_type_bits,
                properties,
            )?);
        let memory = unsafe { device.allocate_memory(&alloc_info, None)? };
        unsafe { device.bind_image_memory(image, memory, 0)? };

        Ok((image, memory))
    }
}
#[derive(Debug)]
struct QueueFamilyIndices {
    graphics_family: Option<u32>,
    present_family: Option<u32>,
}

impl QueueFamilyIndices {
    fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.present_family.is_some()
    }
}

struct SwapChainSupportDetails {
    capabilities: ash::vk::SurfaceCapabilitiesKHR,
    formats: Vec<ash::vk::SurfaceFormatKHR>,
    present_modes: Vec<ash::vk::PresentModeKHR>,
}

unsafe extern "system" fn debug_callback(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    p_user_data: *mut ffi::c_void,
) -> ash::vk::Bool32 {
    let callback_data = p_callback_data.read();
    let callback = (p_user_data as *mut DebugCallback).read();
    callback(message_severity, message_types, callback_data);

    ash::vk::FALSE
}

impl VulkanApp {
    pub fn cleanup(self) {
        self.vulkan.cleanup();
    }
}
impl VulkanData {
    unsafe fn cleanup_swapchain(
        device: &ash::Device,
        swapchain_device: &ash::khr::swapchain::Device,
        framebuffers: Vec<ash::vk::Framebuffer>,
        image_views: Vec<ash::vk::ImageView>,
        swapchain: ash::vk::SwapchainKHR,
    ) {
        unsafe {
            for framebuffer in framebuffers {
                device.destroy_framebuffer(framebuffer, None);
            }
            for image_view in image_views {
                device.destroy_image_view(image_view, None);
            }

            swapchain_device.destroy_swapchain(swapchain, None);
        }
    }
    fn cleanup(self) {
        unsafe {
            for sem in self.image_available_semaphores {
                self.device.destroy_semaphore(sem, None);
            }
            for sem in self.render_finished_semaphores {
                self.device.destroy_semaphore(sem, None);
            }
            for fence in self.in_flight_fences {
                self.device.destroy_fence(fence, None);
            }
        }

        unsafe {
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_pipeline(self.graphics_pipeline, None);
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_render_pass(self.render_pass, None);
            self.device.destroy_shader_module(self.shader_module, None);
        }

        unsafe {
            Self::cleanup_swapchain(
                &self.device,
                &self.swapchain_device,
                self.swap_chain_framebuffers,
                self.swapchain_image_views,
                self.swapchain,
            )
        };

        unsafe {
            self.device.destroy_sampler(self.texture_sampler, None);
            self.device
                .destroy_image_view(self.texture_image_view, None);
            self.device.destroy_image(self.texture_image, None);
            self.device.free_memory(self.texture_image_memory, None);
        }

        unsafe {
            for (buf, mem) in self
                .uniform_buffers
                .into_iter()
                .zip(self.uniform_buffers_memory)
            {
                self.device.destroy_buffer(buf, None);
                self.device.free_memory(mem, None);
            }

            self.device
                .destroy_descriptor_pool(self.descriptor_pool, None);

            self.device
                .destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        }

        unsafe {
            self.device.destroy_buffer(self.index_buffer, None);
            self.device.free_memory(self.index_buffer_memory, None);
            self.device.destroy_buffer(self.vertex_buffer, None);
            self.device.free_memory(self.vertex_buffer_memory, None);
        }

        unsafe {
            self.device.destroy_device(None);
            _ = self.graphics_queue;
            _ = self.present_queue;
            _ = self.device;
        }

        unsafe {
            let surface_khr_instance =
                ash::khr::surface::Instance::new(&self.entry, &self.instance);
            surface_khr_instance.destroy_surface(self.surface, None);
        }

        if let Some(x) = self.debug_callback {
            if let Some(y) = &self.debug_utils_instance {
                unsafe { y.destroy_debug_utils_messenger(x, None) };
            } else {
                eprintln!("Debug utils instance is lost before cleaning up debug callback");
            }
        }

        if let Some(y) = self.debug_utils_instance {
            // No cleanup needed
            _ = y;
        }

        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
