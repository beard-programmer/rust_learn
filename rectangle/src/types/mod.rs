#[derive(Debug)]
pub struct Rectangle {
    pub height: RectanglSide,
    pub width: RectanglSide,
}
pub type RectanglSide = u32;
pub type RectangleArea = u32;