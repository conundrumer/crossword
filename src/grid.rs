use std::fmt::{Display, Formatter, Result};

use placement::{ Position, BoundingBox };
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
    fn row(&self, row: i8) -> i8 {
        row - self.bb.top
    }
    fn col(&self, col: i8) -> i8 {
        col - self.bb.left
    }
    fn row_col(&self, row: i8, col: i8) -> usize {
        let row = self.row(row) as usize;
        let col = self.col(col) as usize;
        let width = self.bb.width() as usize;
        width * row + col
    }
    fn row_col_inverse(&self, i: usize) -> (i8, i8) {
        let i = i as i16;
        let top = self.bb.top as i16;
        let left = self.bb.left as i16;
        let actual_width = self.bb.width() as i16;
        ((i / actual_width + top) as i8, (i % actual_width + left) as i8)
    }
    pub fn set(&self, word: &str, pos: Position) -> Grid {
        let bb = self.bb.combine(BoundingBox::from_word_pos(word, pos).expand());
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
    pub fn can_add_word(&self, word: &str, pos: Position) -> bool {
        GridCell::from_word(word, pos).all(|(cell, (row, col))| {
            let grid_row = self.row(row);
            let grid_col = self.col(col);
            if grid_row < 0 || grid_col < 0 {
                return true
            }
            let row_col = self.row_col(row, col);
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
            let row_col = self.row_col(row, col);
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
