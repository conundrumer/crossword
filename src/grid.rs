use std::fmt::{Display, Formatter, Result};
use std::ops::{Index, IndexMut};

use placement::{ Position, GridIndex, BoundingBox };

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Option<char>>>,
    top: GridIndex,
    left: GridIndex
}
impl Grid {
    pub fn new(bb: &BoundingBox) -> Grid {
        Grid {
            grid: vec![vec![None; bb.width as usize]; bb.height as usize],
            top: bb.top,
            left: bb.left
        }
    }
    fn row(&self, row: GridIndex) -> usize {
        (row - self.top) as usize
    }
    fn col(&self, col: GridIndex) -> usize {
        (col - self.left) as usize
    }
}
impl<'a> Index<&'a Position> for Grid {
    type Output = Option<char>;

    fn index(&self, pos: &Position) -> &Option<char> {
        &self.grid[self.row(pos.row)][self.col(pos.col)]
    }
}
impl<'a> IndexMut<&'a Position> for Grid {
    fn index_mut(&mut self, pos: &Position) -> &mut Option<char> {
        let (row, col) = (self.row(pos.row), self.col(pos.col));
        &mut self.grid[row][col]
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (i, row) in self.grid.iter().enumerate() {
            for entry in row {
                match *entry {
                    Some(c) => try!(write!(f, "{}", c)),
                    None => try!(write!(f, " "))
                }
            }
            if i != self.grid.len() - 1 {
                try!(write!(f, "\n"))
            }
        }
        write!(f, "")
    }
}
