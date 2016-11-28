use std::rc::Rc;

use crossword::Crossword;
use placement::START_POSITION;
use filter::Filter;

pub struct Generator<'a> {
    word_list: Vec<&'a str>,
    filter: Filter
}
impl<'a> Generator<'a> {
    pub fn new(words: Vec<&'a str>, num_areas: usize) -> Generator<'a> {
        Generator {
            word_list: words.clone(),
            filter: Filter::new(num_areas)
        }
    }

    pub fn num_seen(&self) -> usize {
        self.filter.num_seen()
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
        let &Generator {
            ref word_list,
            ref filter
        } = self;
        let bb = crossword.bounding_box();
        let rc_crossword_1 = Rc::new(crossword);
        let rc_crossword_2 = rc_crossword_1.clone();
        let cloned_words = (*words).clone();
        Box::new(cloned_words.into_iter().enumerate()
            .filter_map(|(new_word_index, opt_word)| {
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
                rc_crossword_2.letters().clone().into_iter()
                    .map(move |char1| (w.clone(), char1))
            })
            .flat_map(|((new_word, new_word_index, next_words), char1)| {
                new_word.chars().enumerate().map(move |(i2, c2)| {
                    ((new_word_index, next_words.clone()), char1, (i2, c2))
                })
            })
            .filter_map(|(w, (c1, pos), (i2, c2))| {
                if c1 != c2 {
                    return None
                }
                let next_pos = pos.from_offset(i2 as i8);
                Some((w, next_pos))
            })
            .filter_map(move |((new_word_index, next_words), next_pos)| {
                if filter.by_area(word_list[new_word_index], next_pos, bb) {
                    return Some((new_word_index, next_words, next_pos))
                } else {
                    return None
                }
            })
            .filter_map(move |(new_word_index, next_words, next_pos)| {
                if rc_crossword_1.can_add_word(word_list[new_word_index], next_pos) {
                    Some((rc_crossword_1.set(word_list, new_word_index, next_pos), next_words))
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
        ], 2);
        let crosswords: Vec<_> = gen.iter().collect();

        assert_eq!(11, crosswords.len());
    }
}
