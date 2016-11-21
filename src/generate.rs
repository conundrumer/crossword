use std::collections::HashMap;

use word::{ WordPosition };
use crossword::{ Crossword };
use placement::{ Position, GridIndex };
use placement::Direction::{ Horizontal, Vertical };

pub struct Generator;
impl Generator {
    // simple generator
    pub fn generate(init_words: Vec<&str>, _opts: (usize, GridIndex, GridIndex)) -> Vec<Crossword> {
        if init_words.len() == 0 {
            return vec![];
        }
        let mut cloned_words = init_words.clone();
        let word = cloned_words.remove(0);
        let first_word_pos = WordPosition {
            word: word,
            pos: Position { row: 0, col: 0, dir: Horizontal }
        };
        let crosswords = crosswords_from_word_vec(Crossword { words: vec![first_word_pos] }, cloned_words);
        // remove duplicates using hashmap and string equality
        let crosswords: HashMap<_, _> = crosswords.into_iter().map(|c| (format!("{}", c), c)).collect();
        crosswords.into_iter().map(|(_, c)| c).collect()
    }
}

fn crosswords_from_word_vec<'a> (crossword: Crossword<'a>, words: Vec<&'a str>) -> Vec<Crossword<'a>> {
    if words.len() == 0 {
        return vec![crossword];
    }
    let cloned_words = words.clone();
    words.into_iter().enumerate()
        .flat_map(move |(i, word)| {
            let mut next_words = cloned_words.clone();
            next_words.remove(i);
            crosswords_from_word(crossword.clone(), word)
                .into_iter()
                .flat_map(move |crossword_from_word| crosswords_from_word_vec(crossword_from_word.clone(), next_words.clone()))
        })
        .collect()
}
fn crosswords_from_word<'a> (crossword: Crossword<'a>, new_word: &'a str) -> Vec<Crossword<'a>> {
    crossword.words.clone().into_iter().flat_map(|word_pos| {
        word_pos.chars().into_iter().flat_map(|(c1, pos)| {
            new_word.chars().enumerate().flat_map(|(i, c2)| {
                let i = i as GridIndex;
                if c1 != c2 {
                    return None
                }
                let (row, col, dir) = match word_pos.pos.dir {
                    Horizontal => (pos.row - i, pos.col, Vertical),
                    Vertical => (pos.row, word_pos.pos.col - i, Horizontal)
                };
                let next_crossword = crossword.add(WordPosition {
                    word: new_word,
                    pos: Position { row: row, col: col, dir: dir }
                });
                if next_crossword.is_valid() {
                    Some(next_crossword)
                } else {
                    None
                }
            }).collect::<Vec<_>>().into_iter()
        }).collect::<Vec<_>>().into_iter()
    })
    .collect()
}
