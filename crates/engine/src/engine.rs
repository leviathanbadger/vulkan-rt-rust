use std::fmt::Debug;

use log::info;
use anyhow::*;
use crate::graphics_adapter::GraphicsAdapter;
use crate::platform_adapter::PlatformAdapter;

#[derive(Debug)]
pub struct Engine {
    #[allow(dead_code)]
    platform: Box<dyn PlatformAdapter>,
    #[allow(dead_code)]
    graphics: Box<dyn GraphicsAdapter>
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
        graphics
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
