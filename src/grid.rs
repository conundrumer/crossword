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
    pub letters: Vec<(char, Position)>,
    pub bb: BoundingBox
}
impl Grid {
    pub fn new(bb: BoundingBox) -> Grid {
        Grid {
            is_valid: true,
            num_overlaps: 0,
            grid: Grid::make_grid(bb),
            letters: vec![],
            bb: bb
        }
    }
    fn make_grid(bb: BoundingBox) -> Vec<GridCell> {
        vec![Empty; ((bb.width() as usize) * (bb.height() as usize))]
    }
    pub fn set(&self, word: &str, pos: Position) -> Grid {
        let bb = self.bb.combine(BoundingBox::from_word_pos(word, pos).expand());
        let mut grid = Grid::make_grid(bb);
        let mut letters = self.letters.clone();
        let mut is_valid = self.is_valid;
        let mut num_overlaps = self.num_overlaps;
        // copy old cells to new grid
        for (i, &cell) in self.grid.iter().enumerate() {
            let (row, col) = self.bb.row_col_inverse(i);
            let row_col = bb.row_col(row, col);
            grid[row_col] = cell
        }
        // add word and check for collisions and overlaps and letter additions/removals
        for  (cell, (row, col)) in GridCell::from_word(word, pos) {
            let row_col = bb.row_col(row, col);
            let old_cell = grid[row_col];
            let next_cell = old_cell.get_next(cell);
            grid[row_col] = next_cell;
            match next_cell {
                Letter(c, Some(dir)) => {
                    // add letter
                    letters.push((c, Position {row: row, col: col, dir: dir}));
                },
                Letter(_, None) => if let Letter(_, _) = old_cell {
                    // remove letter
                    letters.iter()
                        .position(|&(_, pos)| (pos.row, pos.col) == (row, col) )
                        .map(|i| letters.remove(i));
                    if let Letter(_, _) = cell {
                        // add overlap
                        num_overlaps += 1;
                    }
                },
                Collision => {
                    is_valid = false
                },
                _ => {}
            }
        }
        Grid {
            is_valid: is_valid,
            num_overlaps: num_overlaps,
            grid: grid,
            letters: letters,
            bb: bb
        }
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
