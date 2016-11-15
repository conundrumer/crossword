use std;

pub type GridIndex = i32;
pub const MAX_INDEX: GridIndex = std::i32::MAX;
pub const MIN_INDEX: GridIndex = std::i32::MIN;

#[derive(Debug)]
pub struct Position {
    pub row: GridIndex,
    pub col: GridIndex
}

#[derive(Debug)]
pub enum Orientation {
    Horizontal,
    Vertical
}

#[derive(Debug)]
pub struct BoundingBox {
    pub top: GridIndex,
    pub left: GridIndex,
    pub width: GridIndex,
    pub height: GridIndex
}
