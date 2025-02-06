mod vulkan_app;
mod result;

use std::process::{exit, ExitCode};
use crate::vulkan_app::VulkanApp;

fn main() -> ExitCode {
    let mut app = VulkanApp::new();

    let res = app.run();
    
    if let Err(e) = res {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    
    ExitCode::SUCCESS
}
