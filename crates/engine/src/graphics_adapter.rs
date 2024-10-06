use std::fmt::{Debug, Display};
use anyhow::*;

use crate::platform_adapter::PlatformAdapter;

pub trait GraphicsAdapter: Display + Debug {
    fn adapter_name(&self) -> String;
    fn is_compatible_with_platform_adapter(&self, platform: &Box<dyn PlatformAdapter>) -> bool;

    fn initialize(&mut self, platform: &Box<dyn PlatformAdapter>) -> Result<()>;
}

#[derive(Debug)]
#[cfg(test)]
pub(crate) struct DummyGraphicsAdapter {
    pub(crate) compatible: bool
}

#[cfg(test)]
impl Default for DummyGraphicsAdapter {
    fn default() -> Self {
        Self {
            compatible: true
        }
    }
}

#[cfg(test)]
impl GraphicsAdapter for DummyGraphicsAdapter {
    fn adapter_name(&self) -> String {
        "DummyGraphicsAdapter".to_owned()
    }

    fn is_compatible_with_platform_adapter(&self, _platform: &Box<dyn PlatformAdapter>) -> bool {
        self.compatible
    }
}

#[cfg(test)]
impl Display for DummyGraphicsAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.adapter_name())
    }
}
