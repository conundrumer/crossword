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

use std::fmt::{Display, Formatter, Result};
impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{},{},{}", self.row, self.col, match self.dir {
            Horizontal => 'H',
            Vertical => 'V'
        })
    }
}
