use std::rc::Rc;

use crossword::Crossword;
use placement::START_POSITION;
use filter::Filter;

pub struct Generator<'a> {
    word_list: Vec<&'a String>,
    word_chars_list: Vec<Vec<char>>,
    filter: Filter
}
impl<'a> Generator<'a> {
    pub fn new(words: Vec<&'a String>, num_areas: usize) -> Generator<'a> {
        Generator {
            word_list: words.clone(),
            word_chars_list: words.iter().map(|word| word.chars().collect()).collect(),
            filter: Filter::new(num_areas)
        }
    }

    pub fn iter<'b>(&'b self) -> Box<Iterator<Item=Crossword> + 'b> {
        let candidates = (1..self.word_list.len()).collect();
        let first_word = self.word_list[0];
        let first_word_len = self.word_chars_list[0].len();
        let init_crossword = Crossword::new(self.word_list.len()).set(first_word, first_word_len, 0, START_POSITION);

        self.from_word_vec_recursive(init_crossword, Rc::new(candidates))
    }

    fn from_word_vec_recursive<'b>(&'b self, crossword: Crossword, candidates: Rc<Vec<usize>>) -> Box<Iterator<Item=Crossword> + 'b> {
        let n = candidates.len();
        if n == 0 {
            return Box::new(Some(crossword).into_iter());
        }
        Box::new(self.from_word_vec(crossword, candidates)
            .flat_map(move |(next_crossword, next_candidates)| {
                self.from_word_vec_recursive(next_crossword, next_candidates)
            }))
    }

    fn from_word_vec<'b>(&'b self, crossword: Crossword, candidates: Rc<Vec<usize>>) -> Box<Iterator<Item=(Crossword, Rc<Vec<usize>>)> + 'b> {
        let &Generator {
            ref filter,
            ref word_list,
            ref word_chars_list
        } = self;
        let n = candidates.len();
        let bb = crossword.bounding_box();
        let letters = Rc::new(crossword.letters().clone());
        let letters_len = letters.len();
        let crossword = Rc::new(crossword);
        let rc_candidates = candidates.clone();
        Box::new((0..n)
            .map(move |candidate_index| {
                let word_index = rc_candidates[candidate_index];
                let word_len = word_chars_list[word_index].len();
                (word_index, word_len, candidate_index)
            })
            .flat_map(move |w| {
                let letters = letters.clone();
                (0..letters_len).map(move |i| (w, letters[i]))
            })
            .flat_map(move |((word_index, word_len, candidate_index), char_pos)| {
                (0..word_len).map(move |i2| {
                    ((word_index, word_len, candidate_index), char_pos, i2)
                })
            })
            .filter_map(move |((word_index, word_len, candidate_index), (c1, pos), i2)| {
                let word_chars = &word_chars_list[word_index];
                let c2 = word_chars[i2];
                if c1 != c2 {
                    return None
                }
                let word = word_list[word_index];
                let next_pos = pos.from_offset(i2 as i8);
                if !filter.by_area(word_len, next_pos, bb) {
                    return None
                }
                if !crossword.can_add_word(word, word_len, next_pos) {
                    return None
                }
                let next_crossword = crossword.set(word, word_len, word_index, next_pos);
                if !filter.by_seen(&next_crossword, n) {
                    return None
                }
                let mut next_candidates = (*candidates).clone();
                next_candidates.remove(candidate_index);
                let next_candidates = Rc::new(next_candidates);
                Some((next_crossword, next_candidates))
            }))
    }
}
use std::fmt::{Display, Formatter, Result};
impl<'a> Display for Generator<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "word_list:\n")?;
        for word in &self.word_list {
            write!(f, "  - {}\n", word)?;
        }
        write!(f, "num_areas: {}", self.filter.num_areas())?;
        write!(f, "\n")
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use placement::{ Position };
    use placement::Direction::{ Horizontal, Vertical };
    use crossword::tests::make_crossword;

    type WordPosition = (&'static str, Position);

    pub fn test_generator(words: Vec<&str>, num_areas: usize, test_fn: &Fn(Generator) -> ()) {
        let words = words.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let words = words.iter().map(|s| s).collect();
        let gen = Generator::new(words, num_areas);
        test_fn(gen);
    }


    #[test]
    fn display() {
        let words = vec![
            "hello",
            "world"
        ];
        let expected = include_str!("test_generator_display.yaml");
        test_generator(words, 0, &|gen| {
            assert_eq!(expected, format!("{}", gen));
        });
    }

    #[test]
    fn test_gen_iter () {
        let words = vec![
            "ton",
            "tok",
            "nob",
            "kob"
        ];
        let expected = make_crossword(vec![
            ("ton", Position { row: 0, col: 0, dir: Horizontal }),
            ("tok", Position { row: 0, col: 0, dir: Vertical }),
            ("nob", Position { row: 0, col: 2, dir: Vertical }),
            ("kob", Position { row: 2, col: 0, dir: Horizontal })
        ]);
        test_generator(words, 0, &|gen| {
            let crosswords: Vec<_> = gen.iter().collect();
            assert_eq!(1, crosswords.len());
            assert_eq!(expected, crosswords[0]);
        });

        let words = vec![
            "toon",
            "took",
            "noob",
            "koob"
        ];
        test_generator(words, 0, &|gen| {
            let crosswords: Vec<_> = gen.iter().collect();
            assert_eq!(22, crosswords.len());
        });

        let words = vec![
            "toon",
            "took",
            "noob",
            "koob"
        ];
        test_generator(words, 1, &|gen| {
            let crosswords: Vec<_> = gen.iter().collect();
            assert_eq!(10, crosswords.len());
        });
    }

    #[test]
    fn letter_block_collision() {
        let words = vec![
            "1A",
            "B1B2",
            "4CC3",
            "4DD",
            "3EE2",
        ];
        test_generator(words, 0, &|gen| {
            assert_eq!(0, gen.iter().count());
        });
    }
}
