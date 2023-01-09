#![warn(clippy::pedantic)]

pub mod ecs;
pub mod graphics;
pub mod window;
pub mod winit;

#[derive(Default)]
pub struct ButterEngine {
    settings: Settings,
    graphic_state: Option<graphics::State>,
}

impl ButterEngine {
    pub(crate) fn settings(&self) -> &Settings {
        &self.settings
    }

    pub(crate) fn set_graphic_state(&mut self, graphic_state: graphics::State) {
        self.graphic_state = Some(graphic_state);
    }

    /// Renders
    ///
    /// # Panics
    ///
    /// Will panic if the graphic state is not set
    pub(crate) fn render(&mut self) {
        self.graphic_state.as_mut().unwrap().render();
    }
}

#[derive(Default)]
pub struct ButterEngineBuilder<'a> {
    window_title: Option<&'a str>,
    window_size: Option<window::Size>,
    wasm_canvas_id: Option<&'a str>,
}

impl<'a> ButterEngineBuilder<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_window_title(&mut self, window_title: &'a str) -> &mut Self {
        self.window_title = Some(window_title);
        self
    }

    pub fn with_window_size(&mut self, window_size: window::Size) -> &mut Self {
        self.window_size = Some(window_size);
        self
    }

    pub fn with_wasm_canvas_id(&mut self, wasm_canvas_id: &'a str) -> &mut Self {
        self.wasm_canvas_id = Some(wasm_canvas_id);
        self
    }

    pub fn build(&mut self) -> ButterEngine {
        ButterEngine {
            settings: Settings {
                window_settings: window::Settings {
                    title: self.window_title.unwrap_or("Butter App").into(),
                    size: self.window_size.unwrap_or_default(),
                    wasm_canvas_id: self.wasm_canvas_id.unwrap_or("butter-app").into(),
                },
            },
            graphic_state: None,
        }
    }
}

#[derive(Default)]
pub struct Settings {
    pub window_settings: window::Settings,
}
