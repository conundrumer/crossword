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
        let (top, left, bottom, right) = self.word_positions().iter().fold(
            (MAX_INDEX, MAX_INDEX, MIN_INDEX, MIN_INDEX),
            |(top, left, bottom, right), &(word, pos)| {
                let last_pos = pos.letter_pos((word.len() - 1) as GridIndex);
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

#[cfg(test)]
mod tests {
    use super::*;
    use placement::{ Position, BoundingBox };
    use placement::Direction::{ Horizontal, Vertical };

    type WordPosition = (&'static str, Position);

    fn make_crossword(word_positions: Vec<WordPosition>) -> Crossword {
        let (word_list, positions): (Vec<_>, Vec<_>) = word_positions.iter().cloned().unzip();
        Crossword {
            word_list: word_list,
            positions: positions.into_iter().map(|x| Some(x)).collect()
        }
    }

    //   0 1 2 3 4
    // 0
    // 1
    // 2
    // 3 h e l l o
    // 4
    fn make_hello() -> WordPosition {
        ("hello", Position { row: 3, col: 0, dir: Horizontal })
    }

    //   0 1 2 3 4
    // 0     w
    // 1     o
    // 2     r
    // 3     l
    // 4     d
    fn make_world() -> WordPosition {
        ("world", Position { row: 0, col: 2, dir: Vertical })
    }

    //   0 1 2 3 4
    // 0     w
    // 1     o
    // 2     r
    // 3 h e l l o
    // 4     d
    fn make_hello_world() -> Crossword<'static> {
        make_crossword(vec![make_hello(), make_world()])
    }

    #[test]
    fn bounding_box() {
        let crossword = make_hello_world();
        let bb = crossword.bounding_box();
        assert_eq!(BoundingBox { top: 0, left: 0, bottom: 4, right: 4 }, bb);
    }

    #[test]
    fn to_grid() {
        let expected = "       \n   w   \n   o   \n   r   \n hello \n   d   \n       ";
        let crossword = make_hello_world();
        println!("Expected:");
        println!("{}", expected);
        println!("Actual:");
        println!("{}", crossword);
        assert_eq!(expected, format!("{}", crossword));
    }

    #[test]
    fn is_valid() {
        let crossword = make_hello_world();
        assert!(crossword.is_valid());
    }

    //   0 1 2 3 4
    // 0
    // 1
    // 2   n
    // 3   a
    // 4   g
    fn make_nag() -> WordPosition {
        ("nag", Position { row: 2, col: 1, dir: Vertical })
    }

    //   0 1 2 3 4
    // 0       b
    // 1       y
    // 2       e
    // 3
    // 4
    fn make_bye() -> WordPosition {
        ("bye", Position { row: 0, col: 3, dir: Vertical })
    }

    //   0 1 2 3 4
    // 0
    // 1
    // 2     n o
    // 3
    // 4
    fn make_no() -> WordPosition {
        ("no", Position { row: 2, col: 2, dir: Horizontal })
    }

    //   0 1 2 3 4
    // 0
    // 1
    // 2
    // 3 h e y
    // 4
    fn make_hey() -> WordPosition {
        ("hey", Position { row: 3, col: 0, dir: Horizontal })
    }

    #[test]
    fn is_invalid() {
        //   0 1 2 3 4
        // 0
        // 1
        // 2   n
        // 3 h æ l l o
        // 4   g
        let invalid_crossword = make_crossword(vec![make_hello(), make_nag()]);
        assert!(!invalid_crossword.is_valid());
    }

    #[test]
    fn is_invalid_adjacent() {

        //   0 1 2 3 4
        // 0
        // 1
        // 2     n o
        // 3 h e l l o
        // 4
        let adjacent_crossword = make_crossword(vec![make_hello(), make_no()]);
        assert!(!adjacent_crossword.is_valid());
    }

    #[test]
    fn is_invalid_touching() {
        //   0 1 2 3 4
        // 0       b
        // 1       y
        // 2       e
        // 3 h e l l o
        // 4
        let touching_crossword = make_crossword(vec![make_hello(), make_bye()]);
        assert!(!touching_crossword.is_valid());
    }

    #[test]
    fn is_valid_diagonal() {
        //   0 1 2 3 4
        // 0       b
        // 1       y
        // 2       e
        // 3 h e y
        // 4
        let diagonal_crossword = make_crossword(vec![make_hey(), make_bye()]);
        assert!(diagonal_crossword.is_valid());
    }

}
