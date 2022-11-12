pub struct Settings<'a> {
    pub title: &'a str,
    pub size: Size,
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}
