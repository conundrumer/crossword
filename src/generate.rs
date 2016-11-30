use std::rc::Rc;

use crossword::Crossword;
use placement::START_POSITION;
use filter::Filter;

pub struct Generator<'a> {
    word_list: Vec<&'a String>,
    filter: Filter
}
impl<'a> Generator<'a> {
    pub fn new(words: Vec<&'a String>, num_areas: usize) -> Generator<'a> {
        Generator {
            word_list: words.clone(),
            filter: Filter::new(num_areas)
        }
    }

    pub fn iter<'b>(&'b self) -> Box<Iterator<Item=Crossword> + 'b> {
        let mut remaining_words: Vec<_> = self.word_list.iter().cloned().map(|x| Some(x)).collect();
        remaining_words[0] = None;
        let first_word = self.word_list[0];
        let init_crossword = Crossword::new(self.word_list.len()).set(first_word, first_word.chars().count(), 0, START_POSITION);

        self.from_word_vec(init_crossword, Rc::new(remaining_words))
    }

    fn from_word_vec<'b>(&'b self, crossword: Crossword, words: Rc<Vec<Option<&'a String>>>) -> Box<Iterator<Item=Crossword> + 'b> {
        let num_remaining_words = words.iter().filter(|opt_word| opt_word.is_some()).count();
        if num_remaining_words == 0 {
            return Box::new(Some(crossword).into_iter());
        }
        let &Generator {
            ref word_list,
            ref filter
        } = self;
        let bb = crossword.bounding_box();
        let rc_crossword_1 = Rc::new(crossword);
        let rc_crossword_2 = rc_crossword_1.clone();
        let cloned_words = (*words).clone();
        Box::new(cloned_words.into_iter().enumerate()
            .filter_map(|(word_index, opt_word)| {
                opt_word.map(|word| {
                    (word_index, word)
                })
            })
            .map(move |(word_index, word)| {
                let mut next_words = (*words).clone();
                next_words[word_index] = None;
                let next_words = Rc::new(next_words);
                (word, word_index, next_words)
            })
            .flat_map(move |w| {
                rc_crossword_2.letters().clone().into_iter()
                    .map(move |char1| (w.clone(), char1))
            })
            .flat_map(|((word, word_index, next_words), char1)| {
                word.chars().enumerate().map(move |(i2, c2)| {
                    ((word_index, next_words.clone()), char1, (i2, c2))
                })
            })
            .filter_map(|(w, (c1, pos), (i2, c2))| {
                if c1 != c2 {
                    return None
                }
                let next_pos = pos.from_offset(i2 as i8);
                Some((w, next_pos))
            })
            .filter_map(move |((word_index, next_words), next_pos)| {
                let word = word_list[word_index];
                if filter.by_area(word.chars().count(), next_pos, bb) {
                    Some((word_index, next_words, next_pos))
                } else {
                    None
                }
            })
            .filter_map(move |(word_index, next_words, next_pos)| {
                let word = word_list[word_index];
                if rc_crossword_1.can_add_word(word, word.chars().count(), next_pos) {
                    Some((rc_crossword_1.set(word, word.chars().count(), word_index, next_pos), next_words))
                } else {
                    None
                }
            })
            .filter_map(move |(next_crossword, next_words)| {
                if filter.by_seen(&next_crossword, num_remaining_words) {
                    Some((next_crossword, next_words))
                } else {
                    None
                }
            })
            .flat_map(move |(next_crossword, next_words)| {
                self.from_word_vec(next_crossword, next_words)
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
