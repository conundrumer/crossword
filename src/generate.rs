use std::collections::HashSet;
use std::cell::RefCell;
use std::rc::Rc;

use crossword::{ Crossword };
use placement::{ Position, GridIndex };
use placement::Direction::{ Horizontal, Vertical };
use word_placements::WordPlacements;

type SeenWordPlacements = Rc<RefCell<HashSet<WordPlacements>>>;

pub struct Generator<'a> {
    word_list: Vec<&'a str>,
    seen: HashSet<WordPlacements>
}
impl<'a> Generator<'a> {
    // simple generator
    pub fn generate(words: Vec<&str>, (_n, _width): (usize, GridIndex)) -> Vec<Crossword> {
        if words.len() == 0 {
            return vec![];
        }
        let mut remaining_words: Vec<_> = words.clone().into_iter().map(|x| Some(x)).collect();
        remaining_words[0] = None;

        let mut gen = Generator {
            word_list: words.clone(),
            seen: HashSet::new()
        };

        gen.from_word_vec(Crossword::new(words.clone()), &remaining_words)
    }

    fn from_word_vec(&mut self, crossword: Crossword<'a>, words: &Vec<Option<&'a str>>) -> Vec<Crossword<'a>> {
        if words.iter().all(|opt_word| opt_word.is_none()) {
            return vec![crossword];
        }
        let cloned_words = words.clone();
        let self_cell = RefCell::new(self);
        cloned_words.into_iter().enumerate()
            .flat_map(|(new_word_index, opt_word)| {
                opt_word.map(|new_word| {
                    (new_word_index, new_word)
                })
            })
            .map(|(new_word_index, new_word)| {
                let mut next_words = words.clone();
                next_words[new_word_index] = None;
                let next_words = Rc::new(next_words);
                (new_word, new_word_index, next_words)
            })
            .flat_map(|w| {
                crossword.positions.index_positions()
                    .map(|(word_index, word_pos)| {
                        let word = self_cell.borrow().word_list[word_index];
                        (word, word_pos)
                    })
                    .map(move |(word, word_pos)| {
                        (w.clone(), (word, word_pos))
                    })
            })
            .flat_map(|(w, (word, word_pos))| {
                word.chars().enumerate()
                    .map(move |(i1, c1)| {
                        let pos = word_pos.letter_pos(i1 as GridIndex);
                        (w.clone(), (pos, c1))
                    })
            })
            .flat_map(|((new_word, new_word_index, next_words), char1)| {
                new_word.chars().enumerate().map(move |(i2, c2)| {
                    ((new_word_index, next_words.clone()), char1, (i2, c2))
                })
            })
            .flat_map(|(w, (pos, c1), (i2, c2))| {
                if c1 != c2 {
                    return None
                }
                let next_pos = next_pos_from_offset(pos, i2 as GridIndex);
                Some((w, next_pos))
            })
            .flat_map(|((new_word_index, next_words), next_pos)| {
                let next_crossword = crossword.set(new_word_index, next_pos);
                let mut seen = &mut self_cell.borrow_mut().seen;
                if seen.contains(&next_crossword.positions) {
                    return None
                }

                // is_valid is very expensive atm so it's cheaper to first check if it's seen
                seen.insert(next_crossword.positions.clone());
                if next_crossword.is_valid() {
                    Some((next_crossword, next_words))
                } else {
                    None
                }
            })
            .flat_map(|(crossword_from_word, next_words)| {
                self_cell.borrow_mut().from_word_vec(crossword_from_word, &next_words).into_iter()
            })
            .collect()
    }
}
fn next_pos_from_offset(pos: Position, i: GridIndex) -> Position {
    match pos.dir {
        Horizontal => Position { row: pos.row - i, col: pos.col, dir: Vertical },
        Vertical => Position { row: pos.row, col: pos.col - i, dir: Horizontal }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use placement::{ Position };
    use placement::Direction::{ Horizontal, Vertical };
    use crossword::{ Crossword };
    use word_placements::WordPlacements;

    type WordPosition = (&'static str, Position);

    fn make_crossword(word_positions: Vec<WordPosition>) -> Crossword {
        let (word_list, positions): (Vec<_>, Vec<_>) = word_positions.iter().cloned().unzip();
        let n = positions.len();
        Crossword {
            word_list: word_list,
            positions: positions.into_iter().enumerate().fold(
                WordPlacements::new(n),
                |wp, (i, x)| wp.set(i, x)
            )
        }
    }

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

}
