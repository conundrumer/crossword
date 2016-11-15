use std::fmt::{Display, Formatter, Result};
use std::ops::{Index, IndexMut};

use placement::{ Position, GridIndex };

#[derive(Debug)]
pub struct Grid(Vec<Vec<Option<char>>>);
impl Grid {
    pub fn new(width: GridIndex, height: GridIndex) -> Grid {
        Grid(vec![vec![None; width as usize]; height as usize])
    }
}
impl<'a> Index<&'a Position> for Grid {
    type Output = Option<char>;

    fn index(&self, pos: &Position) -> &Option<char> {
        &self.0[pos.row as usize][pos.col as usize]
    }
}
impl<'a> IndexMut<&'a Position> for Grid {
    fn index_mut(&mut self, pos: &Position) -> &mut Option<char> {
        &mut self.0[pos.row as usize][pos.col as usize]
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let &Grid(ref grid) = self;
        for (i, row) in grid.iter().enumerate() {
            for entry in row {
                match *entry {
                    Some(c) => try!(write!(f, "{}", c)),
                    None => try!(write!(f, " "))
                }
            }
            if i != grid.len() - 1 {
                try!(write!(f, "\n"))
            }
        }
        write!(f, "")
    }
}
