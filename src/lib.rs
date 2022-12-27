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
    #[must_use]
    pub fn with_settings(settings: Settings) -> Self {
        Self {
            settings,
            graphic_state: None,
        }
    }

    pub fn settings(&self) -> &Settings {
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
    pub fn render(&mut self) {
        self.graphic_state.as_mut().unwrap().render();
    }
}

#[derive(Default)]
pub struct Settings {
    pub window_settings: window::Settings,
}
