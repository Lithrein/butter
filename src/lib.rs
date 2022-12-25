#![warn(clippy::pedantic)]

pub mod graphics;
pub mod window;
pub mod winit;

#[derive(Default)]
pub struct ButterEngine {
    graphic_state: Option<graphics::State>,
}

impl ButterEngine {
    #[must_use]
    pub fn new() -> Self {
        Self {
            graphic_state: None,
        }
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
