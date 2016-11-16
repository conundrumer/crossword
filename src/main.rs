mod placement;
mod grid;
mod word;
mod crossword;

use placement::{ Position };
use placement::Orientation::{ Horizontal, Vertical };
use word::{ Word };
use crossword::{ Crossword };

fn main() {
    let crossword = make_hello_world();
    println!("{}", crossword);
    println!("{}", crossword.is_valid());
}

//   0 1 2 3 4
// 0
// 1
// 2
// 3 h e l l o
// 4
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

//   0 1 2 3 4
// 0     w
// 1     o
// 2     r
// 3     l
// 4     d
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

//   0 1 2 3 4
// 0     w
// 1     o
// 2     r
// 3 h e l l o
// 4     d
fn make_hello_world() -> Crossword {
    Crossword {
        words: vec![make_hello(), make_world()]
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

#[test]
fn bounding_box() {
    let crossword = make_hello_world();
    let bb = crossword.bounding_box();
    assert_eq!(0, bb.top);
    assert_eq!(0, bb.left);
    assert_eq!(4, bb.bottom);
    assert_eq!(4, bb.right);
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
#[cfg(test)]
fn make_nag() -> Word {
    Word {
        letters: "nag",
        pos: Position {
            row: 2,
            col: 1
        },
        orientation: Vertical
    }
}

//   0 1 2 3 4
// 0       b
// 1       y
// 2       e
// 3
// 4
#[cfg(test)]
fn make_bye() -> Word {
    Word {
        letters: "bye",
        pos: Position {
            row: 0,
            col: 3
        },
        orientation: Vertical
    }
}

//   0 1 2 3 4
// 0
// 1
// 2     n o
// 3
// 4
#[cfg(test)]
fn make_no() -> Word {
    Word {
        letters: "no",
        pos: Position {
            row: 2,
            col: 2
        },
        orientation: Horizontal
    }
}

//   0 1 2 3 4
// 0
// 1
// 2
// 3 h e y
// 4
#[cfg(test)]
fn make_hey() -> Word {
    Word {
        letters: "hey",
        pos: Position {
            row: 3,
            col: 0
        },
        orientation: Horizontal
    }
}

#[test]
fn is_invalid() {
    //   0 1 2 3 4
    // 0
    // 1
    // 2   n
    // 3 h æ l l o
    // 4   g
    let invalid_crossword = Crossword {
        words: vec![make_hello(), make_nag()]
    };
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
    let adjacent_crossword = Crossword {
        words: vec![make_hello(), make_no()]
    };
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
    let touching_crossword = Crossword {
        words: vec![make_hello(), make_bye()]
    };
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
    let diagonal_crossword = Crossword {
        words: vec![make_hey(), make_bye()]
    };
    assert!(diagonal_crossword.is_valid());
}
