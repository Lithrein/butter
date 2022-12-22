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
        #[cfg(target_arch = "wasm32")]
        use winit::platform::web::WindowBuilderExtWebSys;

        let event_loop = EventLoop::new();

        #[allow(unused_mut)]
        let mut window_builder = WindowBuilder::new()
            .with_inner_size(Size::Physical(PhysicalSize {
                width: window_settings.size.width,
                height: window_settings.size.height,
            }))
            .with_title(window_settings.title)
            .with_resizable(false);

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let web_window = web_sys::window().unwrap();
            let document = web_window.document().unwrap();
            let canvas = document
                .get_element_by_id(
                    window_settings
                        .canvas_id
                        .expect("No canvas id specified for the wasm build"),
                )
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .ok();
            window_builder = window_builder.with_canvas(canvas);
        }

        let window = window_builder.build(&event_loop).unwrap();

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