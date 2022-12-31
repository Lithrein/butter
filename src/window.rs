pub struct Settings {
    pub title: String,
    pub size: Size,
    /// The canvas used in the wasm build
    pub wasm_canvas_id: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            title: String::from("Butter application"),
            size: Size::default(),
            wasm_canvas_id: String::from("butter-application"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }
}
