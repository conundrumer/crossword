use placement::{ Position, BoundingBox };
use placement::Direction::{ Horizontal, Vertical };
use word::{ WordPosition };
use crossword::{ Crossword };

//   0 1 2 3 4
// 0
// 1
// 2
// 3 h e l l o
// 4
fn make_hello() -> WordPosition<'static> {
    WordPosition {
        word: "hello",
        pos: Position { row: 3, col: 0, dir: Horizontal }
    }
}

//   0 1 2 3 4
// 0     w
// 1     o
// 2     r
// 3     l
// 4     d
fn make_world() -> WordPosition<'static> {
    WordPosition {
        word: "world",
        pos: Position { row: 0, col: 2, dir: Vertical }
    }
}

//   0 1 2 3 4
// 0     w
// 1     o
// 2     r
// 3 h e l l o
// 4     d
fn make_hello_world() -> Crossword<'static> {
    Crossword {
        words: vec![make_hello(), make_world()]
    }
}

#[test]
fn last_pos() {
    let hello = make_hello();
    assert_eq!(Position { row: 3, col: 4, dir: Horizontal }, hello.last_pos());

    let world = make_world();
    assert_eq!(Position { row: 4, col: 2, dir: Vertical }, world.last_pos());
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
fn make_nag() -> WordPosition<'static> {
    WordPosition {
        word: "nag",
        pos: Position { row: 2, col: 1, dir: Vertical }
    }
}

//   0 1 2 3 4
// 0       b
// 1       y
// 2       e
// 3
// 4
fn make_bye() -> WordPosition<'static> {
    WordPosition {
        word: "bye",
        pos: Position { row: 0, col: 3, dir: Vertical }
    }
}

//   0 1 2 3 4
// 0
// 1
// 2     n o
// 3
// 4
fn make_no() -> WordPosition<'static> {
    WordPosition {
        word: "no",
        pos: Position { row: 2, col: 2, dir: Horizontal }
    }
}

//   0 1 2 3 4
// 0
// 1
// 2
// 3 h e y
// 4
fn make_hey() -> WordPosition<'static> {
    WordPosition {
        word: "hey",
        pos: Position { row: 3, col: 0, dir: Horizontal }
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

use generate::Generator;
#[test]
fn test_generate () {
    let opts = (1, 5);
    let crosswords = Generator::generate(vec![
        "ton",
        "tok",
        "nob",
        "kob"
    ], opts);
    let expected = Crossword {
        words: vec![
            WordPosition { word: "ton", pos: Position { row: 0, col: 0, dir: Horizontal } },
            WordPosition { word: "nob", pos: Position { row: 0, col: 2, dir: Vertical } },
            WordPosition { word: "kob", pos: Position { row: 2, col: 0, dir: Horizontal } },
            WordPosition { word: "tok", pos: Position { row: 0, col: 0, dir: Vertical } }
        ]
    };
    assert_eq!(1, crosswords.len());
    assert_eq!(format!("{}", crosswords[0]), format!("{}", expected));

    let crosswords = Generator::generate(vec![
        "toon",
        "took",
        "noob",
        "koob"
    ], opts);
    assert_eq!(22, crosswords.len());
}
