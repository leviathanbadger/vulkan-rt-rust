use std::fmt::Debug;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::*;
use crate::graphics_adapter::GraphicsAdapter;
use crate::platform_adapter::PlatformAdapter;

#[derive(Debug)]
pub struct Engine {
    platform: Box<dyn PlatformAdapter>,
    graphics: Box<dyn GraphicsAdapter>,
    shutdown_requested: Arc<AtomicBool>
}

impl Engine {
    pub fn create_shutdown_requested(&self) -> Arc<AtomicBool> {
        self.shutdown_requested.clone()
    }

    pub fn start(&mut self) -> Result<()> {
        self.platform.initialize()?;
        self.graphics.initialize(&self.platform)?;

        self.platform.run_event_loop(self.create_shutdown_requested())?;

        Ok(())
    }
}

pub fn build_engine(platform: impl PlatformAdapter + 'static, graphics: impl GraphicsAdapter + 'static) -> Result<Engine> {
    info!(target: "engine", "Building engine using platform adapter {platform} and graphics adapter {graphics}");

    let platform = Box::new(platform) as Box<dyn PlatformAdapter>;
    let graphics = Box::new(graphics) as Box<dyn GraphicsAdapter>;

    if !graphics.is_compatible_with_platform_adapter(&platform) {
        return Err(anyhow!("Graphics adapter {graphics} is not compatible with platform adapter {platform}"));
    }

    Ok(Engine {
        platform,
        graphics,
        shutdown_requested: Arc::new(AtomicBool::new(false))
    })
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use super::*;
    use crate::graphics_adapter::DummyGraphicsAdapter;
    use crate::platform_adapter::DummyPlatformAdapter;

    #[test]
    fn build_engine_should_abort_if_adapters_are_not_compatible() {
        let platform = DummyPlatformAdapter::default();
        let graphics = DummyGraphicsAdapter {
            compatible: false,
            ..Default::default()
        };

        let engine = build_engine(platform, graphics);

        assert_matches!(engine, Err(_));
    }

    #[test]
    fn build_engine_should_create_engine_if_adapters_are_compatible() {
        let platform = DummyPlatformAdapter::default();
        let graphics = DummyGraphicsAdapter::default();

        let engine = build_engine(platform, graphics);

        assert_matches!(engine, Result::Ok(_));
    }
}
