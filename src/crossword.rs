use word::{ Word };
use placement::{ BoundingBox, GridIndex, MAX_INDEX, MIN_INDEX };
use grid::{ Grid };

#[derive(Debug)]
pub struct Crossword {
    pub words: Vec<Word>
}
impl Crossword {
    pub fn bounding_box(&self) -> BoundingBox {
        use std::cmp::{min, max};
        let (top, left, bottom, right) = self.words.iter().fold(
            (MAX_INDEX, MAX_INDEX, MIN_INDEX, MIN_INDEX),
            |(top, left, bottom, right), word| {
                let last_pos = word.last_pos();
                (min(top, word.pos.row), min(left, word.pos.col), max(bottom, last_pos.row), max(right, last_pos.col))
            }
        );
        BoundingBox {
            top: top,
            left: left,
            width: right - left + 1,
            height: bottom - top + 1
        }
    }

    fn to_valid_grid(&self, validate: bool) -> Option<Grid> {
        let bb = self.bounding_box();
        let mut grid = Grid::new(bb.width, bb.height);
        for word in &self.words {
            for (i, c) in word.letters.chars().enumerate() {
                let letter_pos = word.letter_pos(i as GridIndex);
                if validate {
                    if let Some(x) = grid[&letter_pos] {
                        if x != c {
                            return None
                        }
                    }
                }
                grid[&letter_pos] = Some(c)
            }
        }
        return Some(grid)
    }

    fn to_grid(&self) -> Grid {
        self.to_valid_grid(false).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        match self.to_valid_grid(true) {
            None => false,
            Some(_) => true
        }
    }
}
use std::fmt::{Display, Formatter, Result};
impl Display for Crossword {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_grid())
    }
}
