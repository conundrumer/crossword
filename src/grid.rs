use std::fmt::{Display, Formatter, Result};

use placement::{ Position, GridIndex, BoundingBox, Direction };
use placement::Direction::{ Horizontal, Vertical };

#[derive(Debug, Copy, Clone, PartialEq)]
enum GridCell {
    Empty,
    Block(Option<Direction>),
    Letter(char, Option<Direction>),
    Collision
}
use self::GridCell::*;

#[derive(Debug)]
pub struct Grid {
    pub is_valid: bool,
    pub num_overlaps: GridIndex,
    grid: Vec<GridCell>,
    pub bb: BoundingBox
}
impl Grid {
    pub fn new(bb: BoundingBox) -> Grid {
        Grid {
            is_valid: true,
            num_overlaps: 0,
            grid: vec![Empty; ((2 + bb.width()) * (2 + bb.height())) as usize],
            bb: bb
        }
    }
    fn row(&self, row: GridIndex) -> usize {
        (row - self.bb.top + 1) as usize
    }
    fn col(&self, col: GridIndex) -> usize {
        (col - self.bb.left + 1) as usize
    }
    fn row_col(&self, row: GridIndex, col: GridIndex) -> usize {
        (2 + self.bb.width() as usize) * self.row(row) + self.col(col)
    }
    fn row_col_inverse(&self, i: usize) -> (GridIndex, GridIndex) {
        let i = i as GridIndex;
        let actual_width = 2 + self.bb.width();
        (i / actual_width + self.bb.top - 1, i % actual_width + self.bb.left - 1)
    }
    pub fn set(&self, word: &str, pos: Position) -> Grid {
        let bb = self.bb.combine(BoundingBox::from_word_pos(word, pos));
        let mut new = Grid::new(bb);
        new.num_overlaps = self.num_overlaps;
        for (i, &cell) in self.grid.iter().enumerate() {
            let (row, col) = self.row_col_inverse(i);
            let row_col = new.row_col(row, col);
            new.grid[row_col] = cell

        }
        let collided = new.add_word(word, pos);
        new.is_valid = !collided && new.is_valid;
        new
    }
    pub fn add_word(&mut self, word: &str, pos: Position) -> bool {
        let mut collided = false;
        // startpoint
        collided = collided || self.set_block(pos.letter_pos(-1));
        for (letter_pos, c) in word.chars().enumerate().map(|(j, c)| (pos.letter_pos(j as GridIndex), c)) {
            collided = collided || self.set_char(letter_pos, c);
        }
        // endpoint
        collided = collided || self.set_block(pos.letter_pos(word.len() as GridIndex));
        collided
    }
    pub fn set_block(&mut self, pos: Position) -> bool {
        self.set_cell(pos.row, pos.col, Block(None))
    }
    pub fn set_char(&mut self, pos: Position, c: char) -> bool {
        match pos.dir {
            Horizontal => {
                self.set_cell(pos.row - 1, pos.col, Block(Some(pos.dir)));
                self.set_cell(pos.row + 1, pos.col, Block(Some(pos.dir)));
            },
            Vertical => {
                self.set_cell(pos.row, pos.col - 1, Block(Some(pos.dir)));
                self.set_cell(pos.row, pos.col + 1, Block(Some(pos.dir)));
            }
        }
        self.set_cell(pos.row, pos.col, Letter(c, Some(pos.dir)))
    }
    fn set_cell(&mut self, row: GridIndex, col: GridIndex, cell: GridCell) -> bool {
        let row_col = self.row_col(row, col);
        // cell: Block(Some(_)), Block(None), Letter(_, Some(_))
        // old_cell: Empty, Block(Some(_)), Block(None), Letter(_, Some(_)), Letter(_, None), Collision
        let old_cell = self.grid[row_col];
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
        if let Letter(_, None) = next_cell {
            self.num_overlaps += 1;
        }
        self.grid[row_col] = next_cell;
        next_cell == Collision
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (i, entry) in self.grid.iter().enumerate() {
            match *entry {
                // Empty => try!(write!(f, " ")),
                // Block(Some(Horizontal)) => try!(write!(f, "─")),
                // Block(Some(Vertical)) => try!(write!(f, "│")),
                // Block(None) => try!(write!(f, "┼")),
                Empty | Block(_) => try!(write!(f, " ")),
                Letter(c, _) => try!(write!(f, "{}", c)),
                Collision => try!(write!(f, "*"))
            }
            if i != self.grid.len() - 1 && (i + 1) % (2 + self.bb.width() as usize) == 0 {
                try!(write!(f, "\n"))
            }
        }
        write!(f, "")
    }
}
