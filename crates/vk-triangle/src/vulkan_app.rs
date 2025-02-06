use crate::result::Result;

pub(crate) struct VulkanApp {}

impl VulkanApp {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl VulkanApp {
    pub fn run(&mut self) -> Result<()> {
        self.init_vulkan()?;
        self.main_loop()?;
        self.cleanup()?;
        Ok(())
    }

    fn init_vulkan(&mut self) -> Result<()> {
        Ok(())
    }
    fn main_loop(&mut self) -> Result<()> {
        Ok(())
    }
    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
}
