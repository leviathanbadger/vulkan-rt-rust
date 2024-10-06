use std::sync::atomic::Ordering;
use anyhow::*;

use engine::engine::build_engine;
use graphics_adapter_vulkan::adapter::GraphicsAdapterVulkan;
use platform_adapter_winit::adapter::PlatformAdapterWinit;

#[macro_use] extern crate log;

fn main() -> Result<()> {
    colog::init();

    let platform = PlatformAdapterWinit::default();
    let graphics = GraphicsAdapterVulkan::default();

    let mut engine = build_engine(platform, graphics)?;

    let shutdown_requested = engine.create_shutdown_requested();
    ctrlc::set_handler(move || {
        warn!("Ctrl+C handled. Application will shut down asynchronously.");
        shutdown_requested.store(true, Ordering::SeqCst);
    })?;

    engine.start()?;

    Ok(())
}
