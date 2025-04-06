use crate::result::{Result, err, error};
use glfw::{ClientApiHint, Glfw, PWindow, WindowHint, WindowMode};
use std::collections::BTreeSet;
use std::ffi;
use std::ffi::c_char;
use std::mem::MaybeUninit;
use std::ptr::null;
use std::thread::sleep;
use std::time::Duration;

const VALIDATION_LAYERS: &[*const c_char] = &[c"VK_LAYER_KHRONOS_validation".as_ptr()];
const ENABLE_VALIDATION: bool = cfg!(any(debug_assertions, not(debug_assertions)));

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
    fn init_vulkan(glfw: &Glfw, window: &PWindow) -> Result<VulkanData> {
        // TODO Consider safety arguments of dynamically linking the library, and maybe handle a failure with some nicer logs?
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

        let physical_device = unsafe { Self::pick_physical_device(&entry, &instance, surface)? };
        // Safety: the PhysicalDevice from `pick_physical_device` satisfies `is_device_suitable`
        let (device, graphics_queue, present_queue) =
            unsafe { Self::pick_logical_device(&entry, &instance, physical_device, surface) }?;

        Ok(VulkanData {
            entry,
            instance,
            debug_utils_instance,
            debug_callback,
            surface,
            device,
            graphics_queue,
            present_queue,
        })
    }
    fn main_loop(&mut self) {
        let mut x = 0;
        while !self.window.should_close() {
            self.glfw.poll_events();
            sleep(Duration::from_millis(33));
            x += 1;
            if x == 100 {
                println!("This has gone on long enough!");
                break;
            }
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
            .api_version(ash::vk::API_VERSION_1_3);

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
                    | ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
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
    fn create_surface(instance: &ash::Instance, window: &PWindow) -> Result<ash::vk::SurfaceKHR> {
        let mut surface = MaybeUninit::<ash::vk::SurfaceKHR>::uninit();
        let result = window.create_window_surface(instance.handle(), null(), surface.as_mut_ptr());

        match result.result() {
            Ok(()) => Ok(unsafe { surface.assume_init() }),
            Err(e) => Err(e.into()),
        }
    }

    /// # Safety
    /// `surface` MUST be a valid `VkSurfaceKHR` handle
    /// `surface` MUST be created, allocated, or retrieved from `instance`
    unsafe fn pick_physical_device(
        entry: &ash::Entry,
        instance: &ash::Instance,
        surface: ash::vk::SurfaceKHR,
    ) -> Result<ash::vk::PhysicalDevice> {
        // Safety:
        // - instance is a valid VkInstance
        // - all devices given to `is_device_suitable` are from `enumerate_physical_devices`, so are valid `VkPhysicalDevice` handles
        let device = unsafe {
            let device_list = instance.enumerate_physical_devices()?;
            device_list
                .into_iter()
                .find(|&x| Self::is_device_suitable(entry, instance, x, surface))
        };

        device.ok_or_else(|| err("No suitable device found").into())
    }
    /// # SAFETY
    /// `device` MUST be a valid `VkPhysicalDevice` handle
    /// `surface` MUST be a valid `VkSurfaceKHR` handle
    /// `device` and `surface` MUST be created, allocated, or retrieved from the same `VkInstance` `instance`
    unsafe fn is_device_suitable(
        entry: &ash::Entry,
        instance: &ash::Instance,
        device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
    ) -> bool {
        let device_properties = unsafe { instance.get_physical_device_properties(device) };
        let device_features = unsafe { instance.get_physical_device_features(device) };

        // No features needed for now, left here for future reference to use them though
        let _ = (device_properties, device_features);

        let indices = unsafe { Self::find_queue_families(entry, instance, device, surface) };

        indices.is_complete()
    }
    /// # SAFETY
    /// `device` MUST be a valid `VkPhysicalDevice` handle
    /// `surface` MUST be a valid `VkSurfaceKHR` handle
    /// `device` and `surface` MUST be created, allocated, or retrieved from the same `VkInstance`
    unsafe fn find_queue_families(
        entry: &ash::Entry,
        instance: &ash::Instance,
        device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
    ) -> QueueFamilyIndices {
        let mut indices = QueueFamilyIndices {
            graphics_family: None,
            present_family: None,
        };
        let surface_instance = ash::khr::surface::Instance::new(entry, instance);

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
    /// `device` MUST be a valid `VkPhysicalDevice` handle
    /// `surface` MUST be a valid `VkSurfaceKHR` handle
    /// `device` and `surface` MUST be created, allocated, or retrieved from the same `VkInstance` `instance`
    ///
    /// # Panics
    /// If the device is not suitable (as per `is_device_suitable`), this may panic
    unsafe fn pick_logical_device(
        entry: &ash::Entry,
        instance: &ash::Instance,
        physical_device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
    ) -> Result<(ash::Device, ash::vk::Queue, ash::vk::Queue)> {
        // Safety: `physical_device` is a valid VkPhysicalDevice handle
        let indices =
            unsafe { Self::find_queue_families(entry, instance, physical_device, surface) };

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

        let create_info = ash::vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_features(&device_features)
            .enabled_extension_names(&[]);

        let device = unsafe { instance.create_device(physical_device, &create_info, None) }?;

        let graphics_queue =
            unsafe { device.get_device_queue(indices.graphics_family.unwrap(), 0) };
        let present_queue = unsafe { device.get_device_queue(indices.present_family.unwrap(), 0) };

        Ok((device, graphics_queue, present_queue))
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

unsafe extern "system" fn debug_callback(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _p_user_data: *mut ffi::c_void,
) -> ash::vk::Bool32 {
    let message = unsafe { ffi::CStr::from_ptr((*p_callback_data).p_message) };
    println!("{message_severity:?} {message_types:?}, {message:?}");

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
