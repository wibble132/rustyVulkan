use crate::result::{err, error, Result};
use glfw::{ClientApiHint, Glfw, PWindow, WindowHint, WindowMode};
use std::collections::BTreeSet;
use std::ffi;
use std::mem::MaybeUninit;
use std::ptr::null;
use std::thread::sleep;
use std::time::Duration;

const VALIDATION_LAYERS: &[*const ffi::c_char] = &[c"VK_LAYER_KHRONOS_validation".as_ptr()];
const ENABLE_VALIDATION: bool = cfg!(any(debug_assertions, not(debug_assertions)));
static DEVICE_EXTENSIONS: &[&ffi::CStr] = &[ash::vk::KHR_SWAPCHAIN_NAME];

pub(crate) struct VulkanApp {
    glfw: Glfw,
    window: PWindow,

    vulkan: VulkanData,
}

struct VulkanData {
    pub entry: ash::Entry,
    pub instance: ash::Instance,

    pub debug_utils_instance: Option<ash::ext::debug_utils::Instance>,
    pub debug_callback: Option<ash::vk::DebugUtilsMessengerEXT>,

    pub surface: ash::vk::SurfaceKHR,

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
    pub pipeline_layout: ash::vk::PipelineLayout,
    pub graphics_pipeline: ash::vk::Pipeline,
    pub command_pool: ash::vk::CommandPool,
    pub command_buffer: ash::vk::CommandBuffer,

    pub image_available_semaphore: ash::vk::Semaphore,
    pub render_finished_semaphore: ash::vk::Semaphore,
    pub in_flight_fence: ash::vk::Fence,
}

impl VulkanApp {
    pub(crate) fn new() -> Result<Self> {
        // Initialise
        println!("Creating vulkan app");
        let (glfw, window) = Self::init_window()?;
        let vulkan = Self::init_vulkan(&glfw, &window)?;

        Ok(Self {
            glfw,
            window,
            vulkan,
        })
    }
}

impl VulkanApp {
    pub const WIDTH: u32 = 800;
    pub const HEIGHT: u32 = 600;

    pub fn run(&mut self) {
        self.main_loop();
    }

    fn init_window() -> Result<(Glfw, PWindow)> {
        let callback = |x, y| println!("Callback error while loading glfw: {x}, {y}");
        let mut glfw =
            glfw::init(callback).map_err(|e| err(&format!("Failed to initialise glfw: {e:?}")))?;

        // Disable OpenGL since we want to use Vulkan
        glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
        // Disable resizing until we support recreating the swapchain
        glfw.window_hint(WindowHint::Resizable(false));

        let (window, _) = glfw
            .create_window(Self::WIDTH, Self::HEIGHT, "Vulkan", WindowMode::Windowed)
            .ok_or(err("Failed to create a window"))?;

        Ok((glfw, window))
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
        let physical_device =
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

        let (shader_module, pipeline_layout, graphics_pipeline) =
            Self::create_graphics_pipeline(&device, render_pass)?;

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

        let command_buffer = Self::create_command_buffer(&device, command_pool)?;

        let (image_available_semaphore, render_finished_semaphore, in_flight_fence) =
            Self::create_sync_objects(&device)?;

        Ok(VulkanData {
            entry,
            instance,
            debug_utils_instance,
            debug_callback,
            surface,
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
            pipeline_layout,
            graphics_pipeline,
            command_pool,
            command_buffer,
            image_available_semaphore,
            render_finished_semaphore,
            in_flight_fence,
        })
    }

    fn main_loop(&mut self) {
        let mut x = 0;
        while !self.window.should_close() {
            self.glfw.poll_events();
            self.draw_frame().unwrap();

            sleep(Duration::from_millis(33));
            x += 1;
            if x == 100 {
                println!("This has gone on long enough!");
                break;
            }
        }
        
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
        ash::vk::DebugUtilsMessengerCreateInfoEXT::default()
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
            .pfn_user_callback(Some(debug_callback))
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
    ) -> Result<ash::vk::PhysicalDevice> {
        // Safety:
        // - instance is a valid VkInstance
        // - all devices given to `is_device_suitable` are from `enumerate_physical_devices`, so are valid `VkPhysicalDevice` handles
        let device = unsafe {
            let device_list = instance.enumerate_physical_devices()?;
            device_list
                .into_iter()
                .find(|&x| Self::is_device_suitable(instance, surface_instance, x, surface))
        };

        device.ok_or_else(|| err("No suitable device found").into())
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

        if features12.vulkan_memory_model == ash::vk::FALSE {
            return false;
        }

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

        indices.is_complete() && extensions_supported && swap_chain_adequate
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

        let device_features = ash::vk::PhysicalDeviceFeatures::default();

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
    fn create_image_views(
        device: &ash::Device,
        swap_chain_images: &[ash::vk::Image],
        swap_chain_image_format: ash::vk::Format,
    ) -> Result<Vec<ash::vk::ImageView>> {
        let mut swap_chain_image_views =
            vec![ash::vk::ImageView::default(); swap_chain_images.len()];

        for i in 0..swap_chain_images.len() {
            let create_info = ash::vk::ImageViewCreateInfo::default()
                .image(swap_chain_images[i])
                .view_type(ash::vk::ImageViewType::TYPE_2D)
                .format(swap_chain_image_format)
                .components(
                    ash::vk::ComponentMapping::default()
                        .r(ash::vk::ComponentSwizzle::IDENTITY)
                        .g(ash::vk::ComponentSwizzle::IDENTITY)
                        .b(ash::vk::ComponentSwizzle::IDENTITY)
                        .a(ash::vk::ComponentSwizzle::IDENTITY),
                )
                .subresource_range(
                    ash::vk::ImageSubresourceRange::default()
                        .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1),
                );

            let image_view = unsafe { device.create_image_view(&create_info, None) }
                .map_err(|e| err(&format!("Failed to create image views: {e}")))?;
            swap_chain_image_views[i] = image_view;
        }

        Ok(swap_chain_image_views)
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

        let vertex_input_info = ash::vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&[])
            .vertex_attribute_descriptions(&[]);
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
            .front_face(ash::vk::FrontFace::CLOCKWISE)
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

        let pipeline_layout_info = ash::vk::PipelineLayoutCreateInfo::default();
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
    fn create_command_buffer(
        device: &ash::Device,
        command_pool: ash::vk::CommandPool,
    ) -> Result<ash::vk::CommandBuffer> {
        let alloc_info = ash::vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(ash::vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let command_buffers = unsafe { device.allocate_command_buffers(&alloc_info) }?;
        Ok(command_buffers[0])
    }
    fn create_sync_objects(
        device: &ash::Device,
    ) -> Result<(ash::vk::Semaphore, ash::vk::Semaphore, ash::vk::Fence)> {
        let semaphore_info = ash::vk::SemaphoreCreateInfo::default();
        let fence_info =
            ash::vk::FenceCreateInfo::default().flags(ash::vk::FenceCreateFlags::SIGNALED);

        let s1 = unsafe { device.create_semaphore(&semaphore_info, None) }?;
        let s2 = unsafe { device.create_semaphore(&semaphore_info, None) }?;
        let f = unsafe { device.create_fence(&fence_info, None) }?;

        Ok((s1, s2, f))
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

            self.vulkan.device.cmd_draw(command_buffer, 3, 1, 0, 0);

            self.vulkan.device.cmd_end_render_pass(command_buffer);
        }

        unsafe { self.vulkan.device.end_command_buffer(command_buffer) }?;

        Ok(())
    }
    fn draw_frame(&mut self) -> Result<()> {
        unsafe {
            self.vulkan
                .device
                .wait_for_fences(&[self.vulkan.in_flight_fence], true, u64::MAX)
        }?;
        unsafe {
            self.vulkan
                .device
                .reset_fences(&[self.vulkan.in_flight_fence])?;
        }

        let (image_index, _) = unsafe {
            self.vulkan.swapchain_device.acquire_next_image(
                self.vulkan.swapchain,
                u64::MAX,
                self.vulkan.image_available_semaphore,
                ash::vk::Fence::null(),
            )
        }?;

        unsafe {
            self.vulkan.device.reset_command_buffer(
                self.vulkan.command_buffer,
                ash::vk::CommandBufferResetFlags::default(),
            )?;
            self.record_command_buffer(self.vulkan.command_buffer, image_index)?;
        }

        let wait_semaphores = [self.vulkan.image_available_semaphore];
        let wait_stages = [ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [self.vulkan.command_buffer];
        let signal_semaphores = [self.vulkan.render_finished_semaphore];
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
                self.vulkan.in_flight_fence,
            )
        }?;

        let swapchains = [self.vulkan.swapchain];
        let image_indices = [image_index];
        let present_info = ash::vk::PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);
        unsafe {
            self.vulkan
                .swapchain_device
                .queue_present(self.vulkan.present_queue, &present_info)
        }?;

        Ok(())
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
    _p_user_data: *mut ffi::c_void,
) -> ash::vk::Bool32 {
    let message = unsafe { ffi::CStr::from_ptr((*p_callback_data).p_message) };
    let message = message.to_str().unwrap();
    println!("{message_severity:?}-{message_types:?}: {message}");

    ash::vk::FALSE
}

impl VulkanApp {
    pub fn cleanup(self) {
        self.vulkan.cleanup();
    }
}
impl VulkanData {
    fn cleanup(self) {
        unsafe {
            self.device
                .destroy_semaphore(self.image_available_semaphore, None);
            self.device
                .destroy_semaphore(self.render_finished_semaphore, None);
            self.device.destroy_fence(self.in_flight_fence, None);
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
            for framebuffer in self.swap_chain_framebuffers {
                self.device.destroy_framebuffer(framebuffer, None);
            }
            for image_view in self.swapchain_image_views {
                self.device.destroy_image_view(image_view, None);
            }
        }

        unsafe {
            _ = self.swapchain_images;
            _ = self.swapchain_format;
            _ = self.swapchain_extent;

            self.swapchain_device
                .destroy_swapchain(self.swapchain, None);
            _ = self.swapchain;
            _ = self.swapchain_device;
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
