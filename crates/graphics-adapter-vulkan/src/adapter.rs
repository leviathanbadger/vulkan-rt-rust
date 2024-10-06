use std::fmt::Display;
use anyhow::*;

use engine::{
    graphics_adapter::GraphicsAdapter,
    platform_adapter::PlatformAdapter
};
use vulkanalia::{
    loader::{LibloadingLoader, LIBRARY},
    Entry,
    Instance
};

#[derive(Debug, Default)]
pub struct GraphicsAdapterVulkan {
    entry: Option<Entry>,
    #[allow(dead_code)]
    inst: Option<Instance>
}

impl GraphicsAdapterVulkan {
    fn create_instance(&self) -> Result<Instance> {
        info!("Creating Vulkan instance...");
        Err(anyhow!("Not implemented"))
    }
}

impl GraphicsAdapter for GraphicsAdapterVulkan {
    fn adapter_name(&self) -> String {
        "GraphicsAdapterVulkan".to_owned()
    }

    fn is_compatible_with_platform_adapter(&self, platform: &Box<dyn PlatformAdapter>) -> bool {
        platform.creates_window_handle()
    }

    fn initialize(&mut self, platform: &Box<dyn PlatformAdapter>) -> Result<()> {
        let entry: Entry;
        // let inst: Instance;
        unsafe {
            let loader = LibloadingLoader::new(LIBRARY)?;
            entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;

            let _window = platform.get_window_handle()?;
            // inst = self.create_instance()?;
            let _ = self.create_instance();
            // inst = Self::create_instance(initial_title, &bootstrap_loaders, &window, &mut app_data, &entry)?;
        }

        self.entry = Some(entry);
        // self.inst = Some(inst);

        Ok(())
    }
}

impl Display for GraphicsAdapterVulkan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.adapter_name())
    }
}
