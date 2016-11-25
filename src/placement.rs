use std;

pub type GridIndex = i32;
pub const MAX_INDEX: GridIndex = std::i32::MAX;
pub const MIN_INDEX: GridIndex = std::i32::MIN;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: GridIndex,
    pub col: GridIndex,
    pub dir: Direction
}
impl Position {
    pub fn letter_pos(&self, i: GridIndex) -> Position {
        match self.dir {
            Direction::Horizontal => Position {
                row: self.row,
                col: self.col + i,
                dir: self.dir
            },
            Direction::Vertical => Position {
                row: self.row + i,
                col: self.col,
                dir: self.dir
            }
        }
    }
}
pub const START_POSITION: Position = Position { row: 0, col: 0, dir: Direction::Horizontal };

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Horizontal,
    Vertical
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BoundingBox {
    pub top: GridIndex,
    pub left: GridIndex,
    pub bottom: GridIndex,
    pub right: GridIndex
}
impl BoundingBox {
    pub fn new(top: GridIndex, left: GridIndex, bottom: GridIndex, right: GridIndex) -> BoundingBox {
        BoundingBox {
            top: top,
            left: left,
            bottom: bottom,
            right: right
        }
    }
    pub fn from_word_pos(word: &str, pos: Position) -> BoundingBox {
        let last_pos = pos.letter_pos((word.len() - 1) as GridIndex);
        BoundingBox::new(pos.row, pos.col, last_pos.row, last_pos.col)
    }
    pub fn combine(&self, other: BoundingBox) -> BoundingBox {
        use std::cmp::{min, max};
        BoundingBox::new(
            min(self.top, other.top),
            min(self.left, other.left),
            max(self.bottom, other.bottom),
            max(self.right, other.right)
        )
    }
    pub fn width(&self) -> GridIndex {
        self.right - self.left + 1
    }
    pub fn height(&self) -> GridIndex {
        self.bottom - self.top + 1
    }
    pub fn area(&self) -> GridIndex {
        self.width() * self.height()
    }
}
