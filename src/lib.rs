#![warn(clippy::pedantic)]

use butter_ecs::{system, Ecs};

pub use butter_ecs as ecs;
pub mod graphics;
pub mod window;
pub mod winit;

#[derive(Default)]
pub struct ButterEngine {
    settings: Settings,
    graphic_state: Option<graphics::State>,
    systems: Vec<Box<dyn system::System>>,
    ecs: Ecs,
}

impl ButterEngine {
    pub(crate) fn settings(&self) -> &Settings {
        &self.settings
    }

    pub(crate) fn set_graphic_state(&mut self, graphic_state: graphics::State) {
        self.graphic_state = Some(graphic_state);
    }

    pub(crate) fn update(&mut self) {
        for system in &mut self.systems {
            system.run(&self.ecs);
        }
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
    systems: Vec<Box<dyn system::System>>,
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

    pub fn with_system<S, P>(&mut self, system: S) -> &mut Self
    where
        S: system::Into<P>,
        P: system::Parameter,
        <S as ecs::system::Into<P>>::SystemType: ecs::system::System,
    {
        self.systems.push(Box::new(system.into_system()));
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
            systems: self.systems.drain(..).collect(),
            graphic_state: None,
            ecs: Ecs::new(),
        }
    }
}

#[derive(Default)]
pub struct Settings {
    pub window_settings: window::Settings,
}
