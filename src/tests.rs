use placement::{ Position, BoundingBox };
use placement::Direction::{ Horizontal, Vertical };
// use word::{ WordPosition };
use crossword::{ Crossword };

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
    let expected = make_crossword(vec![
        ("ton", Position { row: 0, col: 0, dir: Horizontal }),
        ("tok", Position { row: 0, col: 0, dir: Vertical }),
        ("nob", Position { row: 0, col: 2, dir: Vertical }),
        ("kob", Position { row: 2, col: 0, dir: Horizontal })
    ]);
    assert_eq!(1, crosswords.len());

    assert_eq!(expected, crosswords[0]);

    let crosswords = Generator::generate(vec![
        "toon",
        "took",
        "noob",
        "koob"
    ], opts);

    // for crossword in &crosswords {
    //     println!("{}", crossword);
    // }
    assert_eq!(22, crosswords.len());
}
