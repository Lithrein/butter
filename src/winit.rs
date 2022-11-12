use winit::{
    dpi::{PhysicalSize, Size},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::{window, ButterEngine};

pub struct ButterRunner;
impl ButterRunner {
    /// Starts the engine with the given window settings
    ///
    /// # Panics
    ///
    /// This may panic if the window creation fails
    pub fn run(_engine: &ButterEngine, window_settings: &window::Settings) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(Size::Physical(PhysicalSize {
                width: window_settings.size.width,
                height: window_settings.size.height,
            }))
            .with_title(window_settings.title)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_poll();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    control_flow.set_exit();
                }
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => {}
            }
        });
    }
}
