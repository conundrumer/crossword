use std::fmt;
use std::cmp;

type Index = i32;

#[derive(Debug)]
struct Crossword {
    words: Vec<Word>
}
impl Crossword {
    fn bounding_box(&self) -> BoundingBox {
        let mut top = Index::max_value();
        let mut left = Index::max_value();
        let mut bottom = Index::min_value();
        let mut right = Index::min_value();
        for &Word { letters, row, col, ref orientation } in &self.words {
            top = cmp::min(top, row);
            left = cmp::min(left, col);
            match *orientation {
                Horizontal => {
                    bottom = cmp::max(bottom, row);
                    right = cmp::max(right, col + letters.len() as Index);
                },
                Vertical => {
                    bottom = cmp::max(bottom, row + letters.len() as Index);
                    right = cmp::max(right, col);
                }
            }
        }
        BoundingBox {
            top: top,
            left: left,
            width: (right - left) as Index,
            height: (bottom - top) as Index
        }
    }

    fn to_grid(&self) -> Vec<Vec<Option<char>>> {
        let bb = self.bounding_box();
        let mut grid = vec![vec![None; bb.width as usize]; bb.height as usize];
        for &Word { letters, row, col, ref orientation } in &self.words {
            let row = row as usize;
            let col = col as usize;
            for (i, c) in letters.chars().enumerate() {
                match *orientation {
                    Horizontal => {
                        grid[row][col + i] = Some(c)
                    },
                    Vertical => {
                        grid[row + i][col] = Some(c)
                    }
                }
            }
        }
        grid
    }

    fn is_valid(&self) -> bool {
        let bb = self.bounding_box();
        let mut grid = vec![vec![None; bb.width as usize]; bb.height as usize];
        for &Word { letters, row, col, ref orientation } in &self.words {
            let row = row as usize;
            let col = col as usize;
            for (i, c) in letters.chars().enumerate() {
                match *orientation {
                    Horizontal => {
                        if let Some(x) = grid[row][col + i] {
                            if x != c {
                                return false
                            }
                        }
                        grid[row][col + i] = Some(c)
                    },
                    Vertical => {
                        if let Some(x) = grid[row + i][col] {
                            if x != c {
                                return false
                            }
                        }
                        grid[row + i][col] = Some(c)
                    }
                }
            }
        }
        return true
    }
}
impl fmt::Display for Crossword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
