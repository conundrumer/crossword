use std;

pub type GridIndex = i32;
pub const MAX_INDEX: GridIndex = std::i32::MAX;
pub const MIN_INDEX: GridIndex = std::i32::MIN;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub row: GridIndex,
    pub col: GridIndex,
    pub dir: Direction
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Horizontal,
    Vertical
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BoundingBox {
    pub top: GridIndex,
    pub left: GridIndex,
    pub right: GridIndex,
    pub bottom: GridIndex
}
impl BoundingBox {
  pub fn new(top: GridIndex, left: GridIndex, bottom: GridIndex, right: GridIndex) -> BoundingBox {
    BoundingBox { top: top, left: left, right: right, bottom: bottom }
  }
  pub fn width(&self) -> GridIndex {
    self.right - self.left + 1
  }
  pub fn height(&self) -> GridIndex {
    self.bottom - self.top + 1
  }
}
