use std::fmt::{Display, Formatter, Result};

use placement::Position;
use bounding_box::BoundingBox;
use grid_cell::GridCell;
use grid_cell::GridCell::*;

#[derive(Debug)]
pub struct Grid {
    pub is_valid: bool,
    pub num_overlaps: i8,
    grid: Vec<GridCell>,
    pub bb: BoundingBox
}
impl Grid {
    pub fn new(bb: BoundingBox) -> Grid {
        Grid {
            is_valid: true,
            num_overlaps: 0,
            grid: Grid::make_grid(bb),
            bb: bb
        }
    }
    fn make_grid(bb: BoundingBox) -> Vec<GridCell> {
        vec![Empty; ((bb.width() as usize) * (bb.height() as usize))]
    }
    pub fn set(&self, word: &str, pos: Position) -> Grid {
        let bb = self.bb.combine(BoundingBox::from_word_pos(word, pos).expand());
        let mut new = Grid::new(bb);
        new.num_overlaps = self.num_overlaps;
        for (i, &cell) in self.grid.iter().enumerate() {
            let (row, col) = self.bb.row_col_inverse(i);
            let row_col = new.bb.row_col(row, col);
            new.grid[row_col] = cell

        }
        let collided = new.add_word(word, pos);
        new.is_valid = !collided && new.is_valid;
        new
    }
    pub fn can_add_word(&self, word: &str, pos: Position) -> bool {
        GridCell::from_word(word, pos).all(|(cell, (row, col))| {
            if row < self.bb.top || col < self.bb.left {
                return true
            }
            let row_col = self.bb.row_col(row, col);
            if row_col >= self.grid.len() {
                return true
            }
            let old_cell = self.grid[row_col];
            let next_cell = old_cell.get_next(cell);
            next_cell != Collision
        })

    }
    pub fn add_word(&mut self, word: &str, pos: Position) -> bool {
        GridCell::from_word(word, pos).any(|(cell, (row, col))| {
            let row_col = self.bb.row_col(row, col);
            let old_cell = self.grid[row_col];
            let next_cell = old_cell.get_next(cell);
            if let Letter(_, None) = next_cell {
                self.num_overlaps += 1;
            }
            self.grid[row_col] = next_cell;
            next_cell == Collision
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (i, entry) in self.grid.iter().enumerate() {
            match *entry {
                // Empty => try!(write!(f, " ")),
                // Block(Some(Horizontal)) => try!(write!(f, "-")),
                // Block(Some(Vertical)) => try!(write!(f, "|")),
                // Block(None) => try!(write!(f, "+")),
                Empty | Block(_) => try!(write!(f, " ")),
                Letter(c, _) => try!(write!(f, "{}", c)),
                Collision => try!(write!(f, "*"))
            }
            if i != self.grid.len() - 1 && (i + 1) % (self.bb.width() as usize) == 0 {
                try!(write!(f, "\n"))
            }
        }
        write!(f, "")
    }
}
