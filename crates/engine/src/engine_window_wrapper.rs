use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

#[derive(Debug, Clone, Copy)]
pub struct EngineWindowWrapper {
    raw_wrapper: RawWindowHandle
}

impl EngineWindowWrapper {
    pub fn create(raw_wrapper: RawWindowHandle) -> Self {
        Self {
            raw_wrapper
        }
    }
}

unsafe impl HasRawWindowHandle for EngineWindowWrapper {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.raw_wrapper
    }
}
