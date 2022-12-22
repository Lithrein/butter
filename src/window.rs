#[derive(Default)]
pub struct Settings<'a> {
    pub title: &'a str,
    pub size: Size,
    /// The canvas used in the wasm build
    pub canvas_id: Option<&'a str>,
}

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
