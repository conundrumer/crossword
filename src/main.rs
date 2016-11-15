type Index = i32;
const MAX_INDEX: Index = std::i32::MAX;
const MIN_INDEX: Index = std::i32::MIN;

type Grid = Vec<Vec<Option<char>>>;

#[derive(Debug)]
struct Crossword {
    words: Vec<Word>
}
impl Crossword {
    fn bounding_box(&self) -> BoundingBox {
        use std::cmp::{min, max};
        let (top, left, bottom, right) = self.words.iter().fold(
            (MAX_INDEX, MAX_INDEX, MIN_INDEX, MIN_INDEX),
            |(top, left, bottom, right), word| {
                let (last_row, last_col) = word.last_pos();
                (min(top, word.row), min(left, word.col), max(bottom, last_row), max(right, last_col))
            }
        );
        BoundingBox {
            top: top,
            left: left,
            width: (right - left),
            height: (bottom - top)
        }
    }

    fn to_valid_grid(&self, validate: bool) -> Option<Grid> {
        let bb = self.bounding_box();
        let mut grid = vec![vec![None; bb.width as usize]; bb.height as usize];
        for word in &self.words {
            for (i, c) in word.letters.chars().enumerate() {
                let (grid_row, grid_col) = word.letter_pos(i as Index);
                let (grid_row, grid_col) = (grid_row as usize, grid_col as usize);
                if validate {
                    if let Some(x) = grid[grid_row][grid_col] {
                        if x != c {
                            return None
                        }
                    }
                }
                grid[grid_row][grid_col] = Some(c)
            }
        }
        return Some(grid)
    }

    fn to_grid(&self) -> Grid {
        self.to_valid_grid(false).unwrap()
    }

    fn is_valid(&self) -> bool {
        match self.to_valid_grid(true) {
            None => false,
            Some(_) => true
        }
    }
}
use std::fmt::{Display, Formatter, Result};
impl Display for Crossword {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let grid = self.to_grid();
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


#[derive(Debug)]
struct Word {
    letters: &'static str,
    row: Index,
    col: Index,
    orientation: Orientation
}
impl Word {
    fn len(&self) -> Index {
        self.letters.len() as Index
    }
    fn last_pos(&self) -> (Index, Index) {
        match self.orientation {
            Horizontal => (self.row, self.col + self.len()),
            Vertical => (self.row + self.len(), self.col)
        }
    }
    fn letter_pos(&self, i: Index) -> (Index, Index) {
        match self.orientation {
            Horizontal => (self.row, self.col + i),
            Vertical => (self.row + i, self.col)
        }
    }
}

#[derive(Debug)]
enum Orientation {
    Horizontal,
    Vertical
}

use Orientation::*;

#[derive(Debug)]
struct BoundingBox {
    top: Index,
    left: Index,
    width: Index,
    height: Index
}

/*
  0 1 2 3 4
0     w
1     o
2     r
3 h e l l o
4     d
 */

fn make_hello_world() -> Crossword {
    let hello = Word {
        letters: "hello",
        row: 3,
        col: 0,
        orientation: Horizontal
    };
    let world = Word {
        letters: "world",
        row: 0,
        col: 2,
        orientation: Vertical
    };
    Crossword {
        words: vec![hello, world]
    }
}

fn main() {
    let crossword = make_hello_world();
    println!("{}", crossword);
    println!("{}", crossword.is_valid());
}

#[test]
fn crossword_bounding_box() {
    let crossword = make_hello_world();
    let bb = crossword.bounding_box();
    assert_eq!(0, bb.top);
    assert_eq!(0, bb.left);
    assert_eq!(5, bb.width);
    assert_eq!(5, bb.height);
}

#[test]
fn to_grid() {
    let expected = "  w  \n  o  \n  r  \nhello\n  d  ";
    let crossword = make_hello_world();
    assert_eq!(expected, format!("{}", crossword));
}

#[test]
fn is_valid() {
    let crossword = make_hello_world();
    assert!(crossword.is_valid());

    let mut invalid_crossword = make_hello_world();
    invalid_crossword.words[0].row = 0;
    assert!(!invalid_crossword.is_valid());
}
