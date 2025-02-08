mod result;
mod vulkan_app;

use crate::vulkan_app::VulkanApp;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut app = VulkanApp::new();

    let res = app.run();

    if let Err(e) = res {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
