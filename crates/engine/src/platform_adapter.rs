use std::{
    fmt::{Debug, Display},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc
    },
    thread,
    time::Duration
};
use anyhow::*;

use crate::engine_window_wrapper::EngineWindowWrapper;

pub trait PlatformAdapter: Display + Debug {
    fn adapter_name(&self) -> String;

    fn initialize(&mut self) -> Result<()>;

    fn run_event_loop(&mut self, shutdown_requested: Arc<AtomicBool>) -> Result<()> {
        info!("In dummy run_event_loop. This should be overridden for any real PlatformAdapter implementation.");

        let t = thread::spawn(move || {
            loop {
                if shutdown_requested.load(Ordering::Relaxed) {
                    info!("Shutdown requested via Ctrl+C or other method. Stopping event loop.");
                    break;
                }
                thread::sleep(Duration::from_millis(3));
            }
        });

        let _ = t.join();

        Ok(())
    }

    fn creates_window_handle(&self) -> bool {
        false
    }

    fn get_window_handle(&self) -> Result<EngineWindowWrapper> {
        Err(anyhow!("This platform adapter ({}) does not create a window handle", self.adapter_name()))
    }
}

#[derive(Debug)]
#[cfg(test)]
pub(crate) struct DummyPlatformAdapter { }

#[cfg(test)]
impl Default for DummyPlatformAdapter {
    fn default() -> Self {
        Self { }
    }
}

#[cfg(test)]
impl PlatformAdapter for DummyPlatformAdapter {
    fn adapter_name(&self) -> String {
        "DummyPlatformAdapter".to_owned()
    }
}

#[cfg(test)]
impl Display for DummyPlatformAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.adapter_name())
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::time::Instant;

    use ntest::timeout;

    use super::*;

    #[test]
    #[timeout(50)]
    fn PlatformAdapter_run_event_loop_should_stop_when_shutdown_requested() -> Result<()> {
        let mut platform = DummyPlatformAdapter::default();
        let shutdown_requested = Arc::new(AtomicBool::new(false));

        let start = Instant::now();

        let thread_shutdown_requested = shutdown_requested.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(25));
            thread_shutdown_requested.store(true, Ordering::SeqCst);
        });

        platform.run_event_loop(shutdown_requested)?;

        let duration = start.elapsed().as_millis();

        assert!(duration >= 20 && duration <= 30, "Fake event loop ran for {duration}ms, should have been 25ms");

        Ok(())
    }
}
