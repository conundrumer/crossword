use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cell::RefCell;
use std::rc::Rc;

use crossword::{ Crossword };
use placement::{ Position, GridIndex, MAX_INDEX };
use placement::Direction::{ Horizontal, Vertical };
use word_placements::WordPlacements;

pub struct Generator<'a> {
    word_list: Vec<&'a str>,
    seen: RefCell<HashSet<WordPlacements>>,
    min_areas: RefCell<BinaryHeap<GridIndex>>
}
impl<'a> Generator<'a> {
    pub fn new(words: Vec<&'a str>, num_areas: usize) -> Generator<'a> {
        Generator {
            word_list: words.clone(),
            seen: RefCell::new(HashSet::new()),
            min_areas: RefCell::new(BinaryHeap::from(vec![MAX_INDEX; num_areas]))
        }
    }
    // simple generator
    pub fn generate(words: Vec<&'a str>, num_areas: usize) -> Vec<Crossword> {
        if words.len() == 0 {
            return vec![];
        }
        let gen = Generator::new(words, num_areas);

        let v = gen.iter().collect();
        v
    }

    pub fn iter<'b>(&'b self) -> Box<Iterator<Item=Crossword<'a>> + 'b> {
        let mut remaining_words: Vec<_> = self.word_list.clone().into_iter().map(|x| Some(x)).collect();
        remaining_words[0] = None;

        self.from_word_vec(Crossword::new(self.word_list.clone()), Rc::new(remaining_words))
    }

    fn from_word_vec<'b>(&'b self, crossword: Crossword<'a>, words: Rc<Vec<Option<&'a str>>>) -> Box<Iterator<Item=Crossword<'a>> + 'b> {
        let num_remaining_words = words.iter().filter(|opt_word| opt_word.is_some()).count();
        if num_remaining_words == 0 {
            return Box::new(Some(crossword).into_iter());
        }
        let no_min_area = self.min_areas.borrow().len() == 0;
        let rc_self_1 = Rc::new(self);
        let rc_self_2 = rc_self_1.clone();
        let rc_self_3 = rc_self_1.clone();
        let rc_self_4 = rc_self_1.clone();
        let rc_crossword_1 = Rc::new(crossword);
        let rc_crossword_2 = rc_crossword_1.clone();
        let cloned_words = (*words).clone();
        Box::new(cloned_words.into_iter().enumerate()
            .flat_map(|(new_word_index, opt_word)| {
                opt_word.map(|new_word| {
                    (new_word_index, new_word)
                })
            })
            .map(move |(new_word_index, new_word)| {
                let mut next_words = (*words).clone();
                next_words[new_word_index] = None;
                let next_words = Rc::new(next_words);
                (new_word, new_word_index, next_words)
            })
            .flat_map(move |w| {
                let rc_self_1 = rc_self_1.clone();
                rc_crossword_2.positions.index_positions()
                    .map(move |(word_index, word_pos)| {
                        let word = rc_self_1.word_list[word_index];
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
            .map(move |((new_word_index, next_words), next_pos)| {
                (rc_crossword_1.set(new_word_index, next_pos), next_words)
            })
            .flat_map(move |(next_crossword, next_words)| {
                if no_min_area {
                    return Some((next_crossword, next_words, 0))
                }
                let heap = rc_self_4.min_areas.borrow();
                let min_area = heap.peek().unwrap();
                let area = next_crossword.bounding_box().area();
                if area > *min_area {
                    return None
                }
                Some((next_crossword, next_words, area))
            })
            .flat_map(move |(next_crossword, next_words, area)| {
                let mut seen = &mut rc_self_2.seen.borrow_mut();
                if seen.contains(&next_crossword.positions) {
                    return None
                }

                if next_crossword.is_valid() {
                    seen.insert(next_crossword.positions.clone());
                    if !no_min_area && num_remaining_words == 1 {

                        let mut heap = rc_self_2.min_areas.borrow_mut();
                        let mut min_area = heap.peek_mut().unwrap();
                        // let area = next_crossword.bounding_box().area();
                        *min_area = area;
                    }

                    Some((next_crossword, next_words))
                } else {
                    None
                }
            })
            .flat_map(move |(next_crossword, next_words)| {
                rc_self_3.from_word_vec(next_crossword, next_words)
            }))

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
        let crosswords = Generator::generate(vec![
            "ton",
            "tok",
            "nob",
            "kob"
        ], 0);
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
        ], 0);

        assert_eq!(22, crosswords.len());
    }
    #[test]
    fn test_generate_iter () {
        let gen = Generator::new(vec![
            "ton",
            "tok",
            "nob",
            "kob"
        ], 0);
        let crosswords: Vec<_> = gen.iter().collect();
        let expected = make_crossword(vec![
            ("ton", Position { row: 0, col: 0, dir: Horizontal }),
            ("tok", Position { row: 0, col: 0, dir: Vertical }),
            ("nob", Position { row: 0, col: 2, dir: Vertical }),
            ("kob", Position { row: 2, col: 0, dir: Horizontal })
        ]);
        assert_eq!(1, crosswords.len());

        assert_eq!(expected, crosswords[0]);

        let gen = Generator::new(vec![
            "toon",
            "took",
            "noob",
            "koob"
        ], 0);
        let crosswords: Vec<_> = gen.iter().collect();

        assert_eq!(22, crosswords.len());

        let gen = Generator::new(vec![
            "toon",
            "took",
            "noob",
            "koob"
        ], 10);
        let crosswords: Vec<_> = gen.iter().collect();

        assert_eq!(16, crosswords.len());
    }
}
