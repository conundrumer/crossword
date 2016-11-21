use std::fmt::{Display, Formatter, Result};

use placement::{ Position, GridIndex, BoundingBox, Orientation };
use placement::Orientation::{ Horizontal, Vertical };

#[derive(Debug, Copy, Clone, PartialEq)]
enum GridCell {
    Empty,
    Block(Option<Orientation>),
    Letter(char, Option<Orientation>),
    Collision
}
use self::GridCell::*;

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<GridCell>>,
    bb: BoundingBox
}
impl Grid {
    pub fn new(bb: BoundingBox) -> Grid {
        Grid {
            grid: vec![vec![Empty; 2 + bb.width() as usize]; 2 + bb.height() as usize],
            bb: bb
        }
    }
    fn row(&self, row: GridIndex) -> usize {
        (row - self.bb.top + 1) as usize
    }
    fn col(&self, col: GridIndex) -> usize {
        (col - self.bb.left + 1) as usize
    }
    // pub fn get(&self, pos: Position) -> Option<char> {
    //     match self.grid[self.row(pos.row)][self.col(pos.col)] {
    //         Letter(c) => Some(c),
    //         _ => None
    //     }
    // }
    pub fn set_block(&mut self, pos: Position) -> bool {
        let (row, col) = (self.row(pos.row), self.col(pos.col));
        self.set_cell(row, col, Block(None))
    }
    pub fn set_char(&mut self, pos: Position, orient: Orientation, c: char) -> bool {
        let (row, col) = (self.row(pos.row), self.col(pos.col));
        match orient {
            Horizontal => {
                self.set_cell(row - 1, col, Block(Some(orient)));
                self.set_cell(row + 1, col, Block(Some(orient)));
            },
            Vertical => {
                self.set_cell(row, col - 1, Block(Some(orient)));
                self.set_cell(row, col + 1, Block(Some(orient)));
            }
        }
        self.set_cell(row, col, Letter(c, Some(orient)))
    }
    fn set_cell(&mut self, row: usize, col: usize, cell: GridCell) -> bool {
        // cell: Block(Some(_)), Block(None), Letter(_, Some(_))
        // old_cell: Empty, Block(Some(_)), Block(None), Letter(_, Some(_)), Letter(_, None), Collision
        let old_cell = self.grid[row][col];
        let next_cell = match (cell, old_cell) {
            (_, Empty) => {
                cell
            },
            (Block(Some(o1)), Block(Some(o2))) if o1 == o2 => {
                Block(Some(o1))
            },
            (Block(_), Block(_)) => {
                Block(None)
            },
            (Letter(c1, Some(o1)), Letter(c2, Some(o2))) if c1 == c2 && o1 != o2 => {
                Letter(c1, None)
            },
            (Letter(c, opt_o1 @ _), Block(opt_o2 @ _)) | (Block(opt_o2 @ _), Letter(c, opt_o1 @ _)) => {
                match (opt_o1, opt_o2) {
                    (Some(o1), Some(o2)) if o1 != o2 => Letter(c, Some(o1)),
                    (None, _) => Letter(c, None),
                    _ => Collision
                }
            },
            (_, _) => match cell {
                Block(_) | Letter(_, Some(_)) => Collision,
                _ => unreachable!()
            }
        };
        self.grid[row][col] = next_cell;
        next_cell == Collision
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (i, row) in self.grid.iter().enumerate() {
            for entry in row {
                match *entry {
                    // Empty => try!(write!(f, " ")),
                    // Block(Some(Horizontal)) => try!(write!(f, "─")),
                    // Block(Some(Vertical)) => try!(write!(f, "│")),
                    // Block(None) => try!(write!(f, "┼")),
                    Empty | Block(_) => try!(write!(f, " ")),
                    Letter(c, _) => try!(write!(f, "{}", c)),
                    Collision => try!(write!(f, "*"))
                }
            }
            if i != self.grid.len() - 1 {
                try!(write!(f, "\n"))
            }
        }
        write!(f, "")
    }
}
