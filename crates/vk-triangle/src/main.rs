#![warn(clippy::all)]

mod result;
mod vulkan_app;

use crate::vulkan_app::VulkanApp;
use std::error::Error;
use std::process::ExitCode;

fn main() -> ExitCode {
    let res = main_impl();

    if let Err(e) = res {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn main_impl() -> Result<(), Box<dyn Error>> {
    let mut app = VulkanApp::new()?;
    app.run();
    app.cleanup();

    Ok(())
}
