use crate::vec2::Vec2;

pub struct Window {
    title: String,
    size: Vec2,
    position: Vec2,
}

trait WindowTrait {
    fn open(&self);
    fn set_title(&self) -> Self;
    fn set_position(&self, pos: Vec2) -> Self;
    fn set_size(&self, size: Vec2) -> Self;
}
