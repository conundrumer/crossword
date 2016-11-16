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
        BoundingBox::new(top, left, bottom, right)
    }

    fn to_valid_grid(&self, validate: bool) -> Option<Grid> {
        let bb = self.bounding_box();
        let mut grid = Grid::new(bb);
        for word in &self.words {
            // TODO: use an extended array/iter lib to append things more cleanly
            let startpoint = Some((word.letter_pos(-1), None)).into_iter();
            let endpoint = Some((word.letter_pos(word.len()), None)).into_iter();
            let chars = word.letters.chars().enumerate().map(|(i, c)| (word.letter_pos(i as GridIndex), Some(c)));

            for (pos, opt_c) in startpoint.chain(chars).chain(endpoint) {
                let collided = match opt_c {
                    None => grid.set_block(pos),
                    Some(c) => grid.set_char(pos, word.orientation, c)
                };
                if validate {
                    if collided {
                        return None
                    }
                }
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
