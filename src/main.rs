type GridIndex = i32;
const MAX_INDEX: GridIndex = std::i32::MAX;
const MIN_INDEX: GridIndex = std::i32::MIN;

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
        write!(f, "{}", self.to_grid())
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<Option<char>>>);
impl Grid {
    fn new(width: GridIndex, height: GridIndex) -> Grid {
        Grid(vec![vec![None; width as usize]; height as usize])
    }
}
use std::ops::{Index, IndexMut};
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

#[derive(Debug)]
struct Word {
    letters: &'static str,
    pos: Position,
    orientation: Orientation
}
impl Word {
    fn len(&self) -> GridIndex {
        self.letters.len() as GridIndex
    }
    fn letter_pos(&self, i: GridIndex) -> Position {
        match self.orientation {
            Horizontal => Position {
                row: self.pos.row,
                col: self.pos.col + i
            },
            Vertical => Position {
                row: self.pos.row + i,
                col: self.pos.col
            }
        }
    }
    fn last_pos(&self) -> Position {
        self.letter_pos((self.len() - 1) as GridIndex)
    }
}

#[derive(Debug)]
struct Position {
    row: GridIndex,
    col: GridIndex
}

#[derive(Debug)]
enum Orientation {
    Horizontal,
    Vertical
}

use Orientation::*;

#[derive(Debug)]
struct BoundingBox {
    top: GridIndex,
    left: GridIndex,
    width: GridIndex,
    height: GridIndex
}

/*
  0 1 2 3 4
0     w
1     o
2     r
3 h e l l o
4     d
 */

fn main() {
    let crossword = make_hello_world();
    println!("{}", crossword);
    println!("{}", crossword.is_valid());
}

fn make_hello() -> Word {
    Word {
        letters: "hello",
        pos: Position {
            row: 3,
            col: 0
        },
        orientation: Horizontal
    }
}

fn make_world() -> Word {
    Word {
        letters: "world",
        pos: Position {
            row: 0,
            col: 2
        },
        orientation: Vertical
    }
}

#[test]
fn last_pos() {
    let hello = make_hello();
    let Position { row, col } = hello.last_pos();
    assert_eq!(3, row);
    assert_eq!(4, col);

    let world = make_world();
    let Position { row, col } = world.last_pos();
    assert_eq!(4, row);
    assert_eq!(2, col);
}

fn make_hello_world() -> Crossword {
    let hello = make_hello();
    let world = make_world();
    Crossword {
        words: vec![hello, world]
    }
}

#[test]
fn bounding_box() {
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
    invalid_crossword.words[0].pos.row = 0;
    assert!(!invalid_crossword.is_valid());
}
