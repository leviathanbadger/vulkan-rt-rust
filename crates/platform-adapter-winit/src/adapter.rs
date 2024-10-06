use std::{
    fmt::Display,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc
    }
};
use anyhow::*;
use raw_window_handle::HasRawWindowHandle;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder}
};

use engine::{
    engine_window_wrapper::EngineWindowWrapper,
    platform_adapter::PlatformAdapter
};

#[derive(Debug, Default)]
pub struct PlatformAdapterWinit {
    event_loop: Option<EventLoop<()>>,
    window: Option<Box<Window>>,
    window_wrapper: Option<EngineWindowWrapper>
}

impl PlatformAdapterWinit {
    pub fn get_window(&self) -> Result<&Box<Window>> {
        self.window.as_ref().context("Window has not been created")
    }
}

impl PlatformAdapter for PlatformAdapterWinit {
    fn adapter_name(&self) -> String {
        "PlatformAdapterWinit".to_owned()
    }

    fn initialize(&mut self) -> Result<()> {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("Vulcan RT Rust")
            .with_inner_size(LogicalSize::new(1920, 1080))
            .build(&event_loop)?;

        self.event_loop = Some(event_loop);
        self.window_wrapper = Some(EngineWindowWrapper::create(window.raw_window_handle()));
        self.window = Some(Box::new(window));

        Ok(())
    }

    fn creates_window_handle(&self) -> bool {
        true
    }

    fn get_window_handle(&self) -> Result<EngineWindowWrapper> {
        self.window_wrapper.context("Window has not been created")
    }

    fn run_event_loop(&mut self, shutdown_requested: Arc<AtomicBool>) -> Result<()> {
        //TODO: Don't abuse Option<> in the struct in order to call run on the event loop without causing an ownership error
        let event_loop = self.event_loop.take().context("run_event_loop called before event loop was created")?;
        let mut destroying = false;

        event_loop.run(move |event, _, control_flow| {
            if destroying {
                *control_flow = ControlFlow::Exit;
                debug!("Window message received after shutdown began: {:?}", event);
                return ();
            }

            *control_flow = ControlFlow::Poll;
            match event {
                Event::MainEventsCleared => {
                    if !destroying && shutdown_requested.load(Ordering::Relaxed) {
                        info!("Shutdown requested via Ctrl+C or other asynchronous method. Shutting down application...");
                        destroying = true;
                    }
                }
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                    info!("Window close requested. Shutting down application...");
                    destroying = true;
                    shutdown_requested.store(true, Ordering::SeqCst); //Notify other threads that may be listening for shutdown
                }
                Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
                    //TODO: pass keyboard input to game state

                    if let Some(keycode) = input.virtual_keycode {
                        if keycode == VirtualKeyCode::Escape {
                            info!("Escape key pressed. Shutting down application...");
                            destroying = true;
                            shutdown_requested.store(true, Ordering::SeqCst); //Notify other threads that may be listening for shutdown
                        }
                    }
                }
                _ => { }
            }

            if destroying {
                *control_flow = ControlFlow::Exit;
            }
        });
    }
}

impl Display for PlatformAdapterWinit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.adapter_name())
    }
}
