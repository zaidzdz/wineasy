use crate::vec2::Vec2;

pub struct Window {
    title: String,
    size: Vec2,
    position: Vec2,
}
impl Default for Window {
    fn default() -> Self {
        Self {
            title: String::from("wineasy window"),
            size: Vec2 { x: 600.0, y: 400.0 },
            position: Vec2 { x: 0.0, y: 0.0 },
        }
    }
}
// Not platform specific
impl Window {
    fn new() -> Self {
        Self::default()
    }
}
pub trait WindowTrait {
    fn open(&self);
    fn set_title(&self) -> Self;
    fn set_position(&self, pos: Vec2) -> Self;
    fn set_size(&self, size: Vec2) -> Self;
}
