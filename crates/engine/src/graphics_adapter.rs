use std::fmt::{Debug, Display};

use crate::platform_adapter::PlatformAdapter;

pub trait GraphicsAdapter: Display + Debug {
    fn adapter_name(&self) -> String;
    fn is_compatible_with_platform_adapter(&self, platform: &Box<dyn PlatformAdapter>) -> bool;
}

#[derive(Debug)]
pub(crate) struct DummyGraphicsAdapter {
    pub(crate) compatible: bool
}

impl Default for DummyGraphicsAdapter {
    fn default() -> Self {
        Self {
            compatible: true
        }
    }
}

impl GraphicsAdapter for DummyGraphicsAdapter {
    fn adapter_name(&self) -> String {
        "DummyGraphicsAdapter".to_owned()
    }

    fn is_compatible_with_platform_adapter(&self, _platform: &Box<dyn PlatformAdapter>) -> bool {
        self.compatible
    }
}

impl Display for DummyGraphicsAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.adapter_name())
    }
}
