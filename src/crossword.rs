use placement::{ Position, BoundingBox, GridIndex, MAX_INDEX, MIN_INDEX };
use placement::Direction::Horizontal;
use grid::{ Grid };

#[derive(Debug)]
pub struct Crossword<'a> {
    pub word_list: Vec<&'a str>,
    pub positions: Vec<Option<Position>>
}
impl<'a> Clone for Crossword<'a> {
    fn clone(&self) -> Self {
        Crossword {
            word_list: self.word_list.clone(),
            positions: self.positions.clone()
        }
    }
}
impl<'a> PartialEq for Crossword<'a> {
    fn eq(&self, other: &Crossword<'a>) -> bool {
        self.positions == other.positions
    }
}
impl<'a> Eq for Crossword<'a> {}
impl<'a> Crossword<'a> {
    pub fn new(word_list: Vec<&'a str>) -> Crossword<'a> {
        let mut positions = vec![None; word_list.len()];
        positions[0] = Some(Position { row: 0, col: 0, dir: Horizontal });
        Crossword {
            word_list: word_list.clone(),
            positions: positions
        }
    }
    pub fn set(&self, word_index: usize, pos: Position) -> Crossword<'a> {
        let mut next_self = self.clone();
        next_self.positions[word_index] = Some(pos);
        next_self
    }
    pub fn bounding_box(&self) -> BoundingBox {
        use std::cmp::{min, max};
        let (top, left, bottom, right) = self.positions.iter().flat_map(|x| x).enumerate().fold(
            (MAX_INDEX, MAX_INDEX, MIN_INDEX, MIN_INDEX),
            |(top, left, bottom, right), (word_index, &pos)| {
                let last_pos = pos.letter_pos((self.word_list[word_index].len() - 1) as GridIndex);
                (min(top, pos.row), min(left, pos.col), max(bottom, last_pos.row), max(right, last_pos.col))
            }
        );
        BoundingBox::new(top, left, bottom, right)
    }
    pub fn word_positions(&self) -> Vec<(&'a str, Position)> {
        self.positions.iter().enumerate()
            .flat_map(|(word_index, opt_pos)|
                opt_pos.map(|pos| (self.word_list[word_index], pos))
            )
            .collect()
    }

    fn to_valid_grid(&self, validate: bool) -> Option<Grid> {
        let bb = self.bounding_box();
        let mut grid = Grid::new(bb);
        for (word, pos) in self.word_positions() {
            // TODO: use an extended array/iter lib to append things more cleanly
            let startpoint = Some((pos.letter_pos(-1), None)).into_iter();
            let endpoint = Some((pos.letter_pos(word.len() as GridIndex), None)).into_iter();
            let chars = word.chars().enumerate().map(|(j, c)| (pos.letter_pos(j as GridIndex), Some(c)));

            for (pos, opt_c) in startpoint.chain(chars).chain(endpoint) {
                let collided = match opt_c {
                    None => grid.set_block(pos),
                    Some(c) => grid.set_char(pos, pos.dir, c)
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
impl<'a> Display for Crossword<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_grid())
    }
}
