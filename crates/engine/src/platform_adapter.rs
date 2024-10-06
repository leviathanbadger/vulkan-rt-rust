use std::fmt::{Debug, Display};

pub trait PlatformAdapter: Display + Debug {
    fn adapter_name(&self) -> String;
}

#[derive(Debug)]
pub(crate) struct DummyPlatformAdapter { }

impl Default for DummyPlatformAdapter {
    fn default() -> Self {
        Self { }
    }
}

impl PlatformAdapter for DummyPlatformAdapter {
    fn adapter_name(&self) -> String {
        "DummyPlatformAdapter".to_owned()
    }
}

impl Display for DummyPlatformAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.adapter_name())
    }
}
