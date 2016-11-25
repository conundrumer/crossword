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
impl GridCell {

    fn from_word<'a>(word: &'a str, pos: Position) -> impl Iterator<Item=(GridCell, (GridIndex, GridIndex))> + 'a {
        let letter_iter = word.chars().enumerate()
            .map(move |(j, c)| {
                (pos.letter_pos(j as GridIndex), c)
            })
            .flat_map(|(pos, c)| {
                let ((row1, col1), (row2, col2)) = match pos.dir {
                    Horizontal => ((pos.row - 1, pos.col), (pos.row + 1, pos.col)),
                    Vertical => ((pos.row, pos.col - 1), (pos.row, pos.col + 1))
                };
                let cell1 = (Block(Some(pos.dir)), (row1, col1));
                let cell2 = (Letter(c, Some(pos.dir)), (pos.row, pos.col));
                let cell3 = (Block(Some(pos.dir)), (row2, col2));
                Some(cell1).into_iter().chain(Some(cell2).into_iter()).chain(Some(cell3).into_iter())
            });
        let start_pos = pos.letter_pos(-1);
        let end_pos = pos.letter_pos(word.len() as GridIndex);
        let start_cell = (Block(None), (start_pos.row, start_pos.col));
        let end_cell = (Block(None), (end_pos.row, end_pos.col));
        Some(start_cell).into_iter().chain(letter_iter).chain(Some(end_cell).into_iter())
    }

    fn get_next(self, cell: GridCell) -> GridCell {
        // cell: Block(Some(_)), Block(None), Letter(_, Some(_))
        // old_cell: Empty, Block(Some(_)), Block(None), Letter(_, Some(_)), Letter(_, None), Collision
        match (cell, self) {
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
        }
    }
}

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
    fn row(&self, row: GridIndex) -> GridIndex {
        (row - self.bb.top + 1)
    }
    fn col(&self, col: GridIndex) -> GridIndex {
        (col - self.bb.left + 1)
    }
    fn row_col(&self, row: GridIndex, col: GridIndex) -> usize {
        let row = self.row(row) as usize;
        let col = self.col(col) as usize;
        (2 + self.bb.width() as usize) * row + col
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
