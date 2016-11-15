mod placement;
mod grid;
mod word;
mod crossword;

use placement::{ Position };
use placement::Orientation::{ Horizontal, Vertical };
use word::{ Word };
use crossword::{ Crossword };
/*
  0 1 2 3 4
0     w
1     o
2     r
3 h e l l o
4     d
 */

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

fn make_hello_world() -> Crossword {
    let hello = make_hello();
    let world = make_world();
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
