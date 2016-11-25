#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Horizontal,
    Vertical
}
use self::Direction::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: i8,
    pub col: i8,
    pub dir: Direction
}
impl Position {
    pub fn letter_pos(&self, i: i8) -> Position {
        match self.dir {
            Horizontal => Position {
                row: self.row,
                col: self.col + i,
                dir: self.dir
            },
            Vertical => Position {
                row: self.row + i,
                col: self.col,
                dir: self.dir
            }
        }
    }
    pub fn from_offset(&self, i: i8) -> Position {
        match self.dir {
            Horizontal => Position {
                row: self.row - i,
                col: self.col,
                dir: Vertical
            },
            Vertical => Position {
                row: self.row,
                col: self.col - i,
                dir: Horizontal
            }
        }
    }
}
pub const START_POSITION: Position = Position { row: 0, col: 0, dir: Horizontal };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BoundingBox {
    pub top: i8,
    pub left: i8,
    pub bottom: i8,
    pub right: i8
}
impl BoundingBox {
    pub fn new(top: i8, left: i8, bottom: i8, right: i8) -> BoundingBox {
        BoundingBox {
            top: top,
            left: left,
            bottom: bottom,
            right: right
        }
    }
    pub fn from_word_pos(word: &str, pos: Position) -> BoundingBox {
        let last_pos = pos.letter_pos((word.len() - 1) as i8);
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
    pub fn combine_word_pos(&self, word: &str, pos: Position) -> BoundingBox {
        self.combine(BoundingBox::from_word_pos(word, pos))
    }
    pub fn width(&self) -> i16 {
        self.right as i16 - self.left as i16 + 1
    }
    pub fn height(&self) -> i16 {
        self.bottom as i16 - self.top as i16 + 1
    }
    pub fn area(&self) -> i16 {
        self.width() * self.height()
    }
}
