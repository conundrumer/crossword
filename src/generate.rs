use std::collections::HashSet;
use std::cell::RefCell;
use std::rc::Rc;

use crossword::{ Crossword };
use placement::{ Position, GridIndex };
use placement::Direction::{ Horizontal, Vertical };
use word_placements::WordPlacements;

type SeenWordPlacements = Rc<RefCell<HashSet<WordPlacements>>>;

pub struct Generator;
impl Generator {
    // simple generator
    pub fn generate(words: Vec<&str>, (_n, _width): (usize, GridIndex)) -> Vec<Crossword> {
        if words.len() == 0 {
            return vec![];
        }
        let mut remaining_words: Vec<_> = words.clone().into_iter().map(|x| Some(x)).collect();
        remaining_words[0] = None;

        let seen_cell = Rc::new(RefCell::new(HashSet::new()));

        crosswords_from_word_vec(Crossword::new(words.clone()), remaining_words, seen_cell)
    }
}

fn crosswords_from_word_vec<'a> (crossword: Crossword<'a>, words: Vec<Option<&'a str>>, seen_cell: SeenWordPlacements) -> Vec<Crossword<'a>> {
    let remaining_words: Vec<_> = words.clone().into_iter().flat_map(|x| x).collect();
    if remaining_words.len() == 0 {
        return vec![crossword];
    }
    let cloned_words = words.clone();
    words.into_iter().enumerate()
        .flat_map(|(word_index, opt_word)| opt_word.map(|word| (word_index, word)))
        .flat_map(move |(word_index, word)| {
            let seen_cell = seen_cell.clone();
            let mut next_words = cloned_words.clone();
            next_words[word_index] = None;
            crosswords_from_word(crossword.clone(), word, word_index, seen_cell.clone())
                .into_iter()
                .flat_map(move |crossword_from_word| crosswords_from_word_vec(crossword_from_word.clone(), next_words.clone(), seen_cell.clone()))
        })
        .collect()
}
fn crosswords_from_word<'a> (crossword: Crossword<'a>, new_word: &'a str, new_word_index: usize, seen_cell: SeenWordPlacements) -> Vec<Crossword<'a>> {
    crossword.positions.index_positions()
    .flat_map(|(word_index, word_pos)| {
        let word = crossword.word_list[word_index];
        word.chars().enumerate().flat_map(|(letter_index, c1)| {
            let pos = word_pos.letter_pos(letter_index as GridIndex);
            new_word.chars().enumerate().flat_map(|(i, c2)| {
                let i = i as GridIndex;
                if c1 != c2 {
                    return None
                }
                let (row, col, dir) = match word_pos.dir {
                    Horizontal => (pos.row - i, pos.col, Vertical),
                    Vertical => (pos.row, pos.col - i, Horizontal)
                };
                let next_crossword = crossword.set(new_word_index, Position { row: row, col: col, dir: dir });
                // is_valid is very expensive atm so it's cheaper to first check if it's seen
                let mut seen = seen_cell.borrow_mut();
                if seen.contains(&next_crossword.positions) {
                    return None
                }
                seen.insert(next_crossword.positions.clone());
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
