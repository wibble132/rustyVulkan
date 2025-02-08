use crate::result::{error, Result};
use glfw::{
    glfwGetRequiredInstanceExtensions, vkCreateInstance, vkEnumerateInstanceExtensionProperties,
    VkApplicationInfo, VkExtensionProperties, VkInstance, VkInstanceCreateInfo,
    VkResult_VK_SUCCESS, VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
    VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO, VK_API_VERSION_1_0, VK_MAKE_VERSION,
};
use std::ffi::{c_int, CStr, CString};
use std::ptr::{addr_of, addr_of_mut, null, null_mut, slice_from_raw_parts};

pub(crate) struct VulkanApp {
    window: *mut glfw::GLFWwindow,
    instance: VkInstance,
}

impl VulkanApp {
    pub(crate) fn new() -> Self {
        Self {
            window: null_mut(),
            instance: null_mut(),
        }
    }
}

impl VulkanApp {
    pub const WIDTH: u32 = 800;
    pub const HEIGHT: u32 = 600;

    pub fn run(&mut self) -> Result<()> {
        self.init_window()?;
        self.init_vulkan()?;
        self.main_loop()?;
        self.cleanup()?;
        Ok(())
    }

    fn init_window(&mut self) -> Result<()> {
        unsafe {
            glfw::glfwInit();
            glfw::glfwWindowHint(glfw::GLFW_CLIENT_API as c_int, glfw::GLFW_NO_API as c_int);
            glfw::glfwWindowHint(glfw::GLFW_RESIZABLE as c_int, glfw::GLFW_FALSE as c_int);

            let name = CString::new("Vulkan").unwrap();
            let window = glfw::glfwCreateWindow(
                Self::WIDTH as c_int,
                Self::HEIGHT as c_int,
                name.as_ptr(),
                null_mut(),
                null_mut(),
            );

            self.window = window;

            Ok(())
        }
    }

    fn init_vulkan(&mut self) -> Result<()> {
        self.create_instance()?;

        Ok(())
    }
    fn main_loop(&mut self) -> Result<()> {
        unsafe {
            while !glfw::glfwWindowShouldClose(self.window) != 0 {
                glfw::glfwPollEvents();
            }

            Ok(())
        }
    }
    fn cleanup(&mut self) -> Result<()> {
        unsafe {
            glfw::glfwDestroyWindow(self.window);

            glfw::glfwTerminate();

            Ok(())
        }
    }
}

impl VulkanApp {
    fn create_instance(&mut self) -> Result<()> {
        let app_name = CString::new("Hello Triangle").unwrap();
        let engine_name = CString::new("No Engine").unwrap();
        let app_info = VkApplicationInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: null(),
            pApplicationName: app_name.as_ptr(),
            applicationVersion: VK_MAKE_VERSION!(1, 0, 0),
            pEngineName: engine_name.as_ptr(),
            engineVersion: VK_MAKE_VERSION!(1, 0, 0),
            apiVersion: VK_API_VERSION_1_0,
        };

        let mut glfw_extension_count = 0;
        let glfw_extensions =
            unsafe { glfwGetRequiredInstanceExtensions(addr_of_mut!(glfw_extension_count)) };

        println!("Required extension count is {glfw_extension_count}");
        let extensions_slice = slice_from_raw_parts(glfw_extensions, glfw_extension_count as usize);
        let extensions = unsafe {
            (*extensions_slice)
                .iter()
                .map(|&p| CStr::from_ptr(p))
                .collect::<Vec<_>>()
        };
        println!("Required extensions are {extensions:?}");

        let mut extension_count = 0;
        unsafe {
            vkEnumerateInstanceExtensionProperties(
                null(),
                addr_of_mut!(extension_count),
                null_mut(),
            );
        }

        let mut extensions = Vec::<VkExtensionProperties>::with_capacity(extension_count as usize);
        unsafe {
            vkEnumerateInstanceExtensionProperties(
                null(),
                addr_of_mut!(extension_count),
                extensions.as_mut_ptr(),
            );
            extensions.set_len(extension_count as usize);
        }
        unsafe {
            for e in extensions {
                let name = CStr::from_ptr(e.extensionName.as_ptr());
                println!("{name:?}")
            }
        }

        let create_info = VkInstanceCreateInfo {
            sType: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: null(),
            flags: 0,
            pApplicationInfo: addr_of!(app_info),

            // No layers needed
            enabledLayerCount: 0,
            ppEnabledLayerNames: null(),

            // Enable all extensions needed for glfw
            enabledExtensionCount: glfw_extension_count,
            ppEnabledExtensionNames: glfw_extensions,
        };

        let result =
            unsafe { vkCreateInstance(addr_of!(create_info), null(), addr_of_mut!(self.instance)) };
        if result != VkResult_VK_SUCCESS {
            return error("failed to create instance");
        }

        Ok(())
    }
}
