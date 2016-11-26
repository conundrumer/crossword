use std;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cell::RefCell;
use std::rc::Rc;

use crossword::{ Crossword };
use placement::{ START_POSITION };
use word_placements::WordPlacements;

pub struct Generator<'a> {
    word_list: Vec<&'a str>,
    seen: RefCell<HashSet<WordPlacements>>,
    min_areas: RefCell<BinaryHeap<i16>>
}
impl<'a> Generator<'a> {
    pub fn new(words: Vec<&'a str>, num_areas: usize) -> Generator<'a> {
        Generator {
            word_list: words.clone(),
            seen: RefCell::new(HashSet::new()),
            min_areas: RefCell::new(BinaryHeap::from(vec![std::i16::MAX; num_areas]))
        }
    }

    pub fn iter<'b>(&'b self) -> Box<Iterator<Item=Crossword> + 'b> {
        let mut remaining_words: Vec<_> = self.word_list.iter().cloned().map(|x| Some(x)).collect();
        remaining_words[0] = None;
        let init_crossword = Crossword::new(&self.word_list).set(&self.word_list, 0, START_POSITION);

        self.from_word_vec(init_crossword, Rc::new(remaining_words))
    }

    fn from_word_vec<'b>(&'b self, crossword: Crossword, words: Rc<Vec<Option<&'a str>>>) -> Box<Iterator<Item=Crossword> + 'b> {
        let num_remaining_words = words.iter().filter(|opt_word| opt_word.is_some()).count();
        if num_remaining_words == 0 {
            return Box::new(Some(crossword).into_iter());
        }
        let no_min_area = self.min_areas.borrow().len() == 0;
        let &Generator {
            ref word_list,
            ref seen,
            ref min_areas
        } = self;
        let bb = crossword.bounding_box();
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
                rc_crossword_2.positions.index_positions()
                    .map(move |(word_index, word_pos)| {
                        let word = word_list[word_index];
                        (word, word_pos)
                    })
                    .map(move |(word, word_pos)| {
                        (w.clone(), (word, word_pos))
                    })
            })
            .flat_map(|(w, (word, word_pos))| {
                word.chars().enumerate()
                    .map(move |(i1, c1)| {
                        let pos = word_pos.letter_pos(i1 as i8);
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
                let next_pos = pos.from_offset(i2 as i8);
                Some((w, next_pos))
            })
            .flat_map(move |((new_word_index, next_words), next_pos)| {
                if no_min_area {
                    return Some((new_word_index, next_words, next_pos))
                }
                let area = bb.combine_word_pos(word_list[new_word_index], next_pos).area();
                let heap = min_areas.borrow();
                let min_area = heap.peek().unwrap();
                if area > *min_area {
                    return None
                }
                Some((new_word_index, next_words, next_pos))
            })
            .flat_map(move |(new_word_index, next_words, next_pos)| {
                if rc_crossword_1.grid.can_add_word(word_list[new_word_index], next_pos) {
                    Some((rc_crossword_1.set(word_list, new_word_index, next_pos), next_words))
                } else {
                    None
                }
            })
            .flat_map(move |(next_crossword, next_words)| {
                let mut seen = &mut seen.borrow_mut();
                if seen.contains(&next_crossword.positions) {
                    return None
                }
                seen.insert(next_crossword.positions.clone());
                if !no_min_area && num_remaining_words == 1 {

                    let mut heap = min_areas.borrow_mut();
                    let mut min_area = heap.peek_mut().unwrap();
                    let area = next_crossword.bounding_box().area();
                    *min_area = area;
                }

                Some((next_crossword, next_words))
            })
            .flat_map(move |(next_crossword, next_words)| {
                self.from_word_vec(next_crossword, next_words)
            }))

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use placement::{ Position };
    use placement::Direction::{ Horizontal, Vertical };
    use crossword::tests::make_crossword;

    type WordPosition = (&'static str, Position);

    #[test]
    fn test_gen_iter () {
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
