use placement::Position;
use bounding_box::BoundingBox;

use grid::Grid;
use word_placements::WordPlacements;

#[derive(Debug, Clone)]
pub struct Crossword {
    pub positions: WordPlacements,
    grid: Grid
}
impl PartialEq for Crossword {
    fn eq(&self, other: &Crossword) -> bool {
        self.positions == other.positions
    }
}
impl Eq for Crossword {}
impl Crossword {
    pub fn new(num_words: usize) -> Crossword {
        Crossword {
            positions: WordPlacements::new(num_words),
            grid: Grid::new(BoundingBox::new(0, 0, 0, 0))
        }
    }
    pub fn can_add_word(&self, word: &str, word_len: usize, pos: Position) -> bool {
        self.grid.can_add_word(word, word_len, pos)
    }
    pub fn set(&self, word: &str, word_len: usize, word_index: usize, pos: Position) -> Crossword {
        Crossword {
            positions: self.positions.set(word_index, pos),
            grid: self.grid.set(word, word_len, pos)
        }
    }
    pub fn bounding_box(&self) -> BoundingBox {
        self.grid.bb.contract()
    }

    pub fn is_valid(&self) -> bool {
        self.grid.is_valid
    }
    pub fn num_overlaps(&self) -> i8 {
        self.grid.num_overlaps
    }
    pub fn letters(&self) -> &Vec<(char, Position)> {
        &self.grid.letters
    }
}
use std::fmt::{Display, Formatter, Result};
impl Display for Crossword {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // portrait by default
        let bb = self.bounding_box();
        let (width, height) = (bb.width(), bb.height());
        let is_landscape = width > height;
        let (width, height) = if is_landscape { (height, width) } else { (width, height) };

        writeln!(f, "[{}]:", self.positions)?;
        writeln!(f, "  width: {}", width)?;
        writeln!(f, "  height: {}", height)?;
        writeln!(f, "  area: {}", width * height)?;
        writeln!(f, "  overlaps: {}", self.num_overlaps())?;

        if is_landscape {
            write_grid(f, "portrait", self.grid.iter_cols())?;
            write_grid(f, "landscape", self.grid.iter_rows())
        } else {
            write_grid(f, "portrait", self.grid.iter_rows())?;
            write_grid(f, "landscape", self.grid.iter_cols())
        }
    }
}
fn write_grid<T, U>(f: &mut Formatter, key_name: &str, iter: T) -> Result
    where T: Iterator<Item=Option<U>>, U: Display {
    writeln!(f, "  {}: |", key_name)?;
    writeln!(f, "    .")?;
    write!(f, "    ")?;
    for entry in iter {
        if let Some(cell) = entry {
            write!(f, "{}", cell)?;
        } else {
            writeln!(f, "")?;
            write!(f, "    ")?;
        }
    }
    writeln!(f, "")
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use placement::Position;
    use bounding_box::BoundingBox;
    use placement::Direction::{ Horizontal, Vertical };

    type WordPosition = (&'static str, Position);

    pub fn make_crossword(word_positions: Vec<WordPosition>) -> Crossword {
        let (word_list, positions): (Vec<_>, Vec<_>) = word_positions.iter().cloned().unzip();
        positions.into_iter().enumerate().fold(
            Crossword::new(word_list.len()),
            |cw, (word_index, pos)| {
                let word = word_list[word_index];
                cw.set(word, word.chars().count(), word_index, pos)
            }
        )
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
    fn make_hello_world() -> Crossword {
        make_crossword(vec![make_hello(), make_world()])
    }

    #[test]
    fn bounding_box() {
        let crossword = make_hello_world();
        let bb = crossword.bounding_box();
        assert_eq!(BoundingBox { top: 0, left: 0, bottom: 4, right: 4 }, bb);
    }

    #[test]
    fn num_overlaps() {
        let crossword = make_hello_world();
        assert_eq!(1, crossword.num_overlaps());
    }

    #[test]
    fn letters() {
        let crossword = make_crossword(vec![make_hello()]);
        assert_eq!(5, crossword.letters().len());

        let crossword = make_hello_world();
        for l in crossword.letters() {
            println!("{:?}", l);
        }
        assert_eq!(4, crossword.letters().len());
    }

    #[test]
    fn display() {
        let expected = include_str!("test_crossword_display.yaml");
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
