use std::rc::Rc;
use std::cell::{RefCell, Cell};

use crossword::Crossword;
use placement::{Position, START_POSITION};
use filter::Filter;
use rand::{hash, rand_range};

pub struct Generator<'a> {
    seed: u64,
    next_seed: Cell<u64>,
    word_list: Vec<&'a String>,
    word_chars_list: Vec<Vec<char>>,
    filter: Filter,
}
impl<'a> Generator<'a> {
    pub fn new(words: Vec<&'a String>, num_areas: usize, seed: u64) -> Generator<'a> {
        Generator {
            seed: seed,
            next_seed: Cell::new(seed),
            word_list: words.clone(),
            word_chars_list: words.iter().map(|word| word.chars().collect()).collect(),
            filter: Filter::new(num_areas)
        }
    }

    fn get_seed(&self) -> u64 {
        let seed = self.next_seed.get();
        self.next_seed.set(hash(seed, self.seed));
        seed
    }

    fn get_init(&self) -> (Crossword, Rc<Vec<usize>>) {
        let first_word = self.word_list[0];
        let first_word_len = self.word_chars_list[0].len();
        let init_crossword = Crossword::new(self.word_list.len()).set(first_word, first_word_len, 0, START_POSITION);
        let candidates = (1..self.word_list.len()).collect();
        (init_crossword, Rc::new(candidates))
    }

    pub fn iter<'b>(&'b self) -> Box<Iterator<Item=Crossword> + 'b> {
        let (init_crossword, candidates) = self.get_init();
        self.from_word_vec_recursive(init_crossword, candidates, self.get_seed())
    }

    pub fn multi_iter<'b>(&'b self, num_iters: usize) -> Box<Iterator<Item=Crossword> + 'b> {
        let (init_crossword, candidates) = self.get_init();
        let seeds = vec![self.get_seed(); num_iters];
        let mut iters = vec![];
        for i in 0..num_iters {
            let iter = self.from_word_vec_partial(init_crossword.clone(), candidates.clone(), seeds[i]);
            iters.push(RefCell::new(iter));
        }
        Box::new((0..num_iters)
            .cycle()
            .scan(false, move |state, i| {
                let mut iter = iters[i].borrow_mut();
                let r = if let Some((crossword, candidates)) = iter.next() {
                    *state = true;
                    Some(Some(self.from_word_vec_recursive(crossword, candidates, seeds[i])))
                } else {
                    Some(None)
                };
                if i == num_iters - 1 {
                    if *state == false {
                        return None
                    } else {
                        *state = false
                    }
                }
                r
            })
            .filter_map(|x| x)
            .flat_map(|x| x)
        )
    }

    fn from_word_vec_partial<'b>(&'b self, crossword: Crossword, candidates: Rc<Vec<usize>>, seed: u64) -> Box<Iterator<Item=(Crossword, Rc<Vec<usize>>)> + 'b> {
        let n = candidates.len();
        if n <= 6 {
            return Box::new(Some((crossword, candidates)).into_iter());
        }
        Box::new(self.from_word_vec(crossword, candidates, seed)
            .flat_map(move |(next_crossword, next_candidates)| {
                self.from_word_vec_partial(next_crossword, next_candidates, seed)
            }))
    }

    fn from_word_vec_recursive<'b>(&'b self, crossword: Crossword, candidates: Rc<Vec<usize>>, seed: u64) -> Box<Iterator<Item=Crossword> + 'b> {
        let n = candidates.len();
        if n == 0 {
            return Box::new(Some(crossword).into_iter());
        }
        Box::new(self.from_word_vec(crossword, candidates, seed)
            .flat_map(move |(next_crossword, next_candidates)| {
                self.from_word_vec_recursive(next_crossword, next_candidates, seed)
            }))
    }

    fn from_word_vec<'b>(&'b self, crossword: Crossword, candidates: Rc<Vec<usize>>, seed: u64) -> Box<Iterator<Item=(Crossword, Rc<Vec<usize>>)> + 'b> {
        let &Generator {
            seed: _,
            next_seed: _,
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
        let seed = if n < 6 { 0 } else { hash(&crossword.positions, seed) };
        let get_words = || {
            (0..n).map(rand_range(n, hash(seed, seed)))
                .map(move |candidate_index| {
                    let word_index = rc_candidates[candidate_index];
                    let word_chars: &Vec<char> = &word_chars_list[word_index];
                    let word_len = word_chars.len();
                    (word_index, word_len, candidate_index)
                })
        };
        let get_letters = move |w| {
            let letters = letters.clone();
            (0..letters_len).map(rand_range(letters_len, hash(w, seed)))
                .map(move |i| (w, letters[i]))
        };
        let get_word_chars = move |((word_index, word_len, candidate_index), char_pos)| {
            (0..word_len).map(rand_range(word_len, hash(char_pos, seed)))
                .map(move |i2| ((word_index, word_len, candidate_index), char_pos, i2))
        };
        let filter_candidates = move |((word_index, word_len, candidate_index), (c1, pos), i2)| {
            let word_chars: &Vec<char> = &word_chars_list[word_index];
            let c2 = word_chars[i2];
            if c1 != c2 {
                return None
            }
            let word: &String = word_list[word_index];
            let pos: Position = pos;
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
        };
        Box::new(
            get_words()
            .flat_map(get_letters)
            .flat_map(get_word_chars)
            .filter_map(filter_candidates)
        )
    }
}
use std::fmt::{Display, Formatter, Result};
impl<'a> Display for Generator<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "word_list:")?;
        for word in &self.word_list {
            writeln!(f, "  - {}", word)?;
        }
        writeln!(f, "num_areas: {}", self.filter.num_areas())?;
        writeln!(f, "seed: {}", self.seed)
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
        let gen = Generator::new(words, num_areas, 0);
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
