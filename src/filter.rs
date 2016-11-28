use std::i16::MAX;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::cell::Cell;
use std::cell::RefCell;

use placement::Position;
use bounding_box::BoundingBox;
use word_placements::WordPlacements;
use crossword::Crossword;

pub struct Filter {
    pub seen: RefCell<HashSet<WordPlacements>>,
    pub min_areas: RefCell<BTreeSet<i16>>,
    min_area: Cell<i16>,
    has_min_areas: bool
}
trait CollectionWithLast<T> {
    fn last(&self) -> Option<T>;
}
impl<T: Copy> CollectionWithLast<T> for BTreeSet<T> {
    fn last(&self) -> Option<T> {
        self.iter().next_back().map(|&x| x)
    }
}
impl Filter {
    pub fn new(num_areas: usize) -> Filter {
        let mut min_areas = BTreeSet::new();
        min_areas.extend((0..num_areas).map(|i| MAX - i as i16));
        Filter {
            seen: RefCell::new(HashSet::new()),
            min_areas: RefCell::new(min_areas),
            min_area: Cell::new(MAX),
            has_min_areas: num_areas > 0
        }
    }
    pub fn num_seen(&self) -> usize {
        self.seen.borrow().len()
    }
    pub fn by_area(&self, word: &str, next_pos: Position, bb: BoundingBox) -> bool {
        if !self.has_min_areas {
            return true
        }
        let area = bb.combine_word_pos(word, next_pos).area();
        area <= self.min_area.get()
    }
    pub fn by_seen(&self, crossword: &Crossword, num_remaining_words: usize) -> bool {
        {
            let mut seen = &mut self.seen.borrow_mut();
            if seen.contains(&crossword.positions) {
                return false
            }
            seen.insert(crossword.positions.clone());
        }
        if self.has_min_areas && num_remaining_words == 1 {
            let min_area = self.min_area.get();
            let mut min_areas = self.min_areas.borrow_mut();

            let area = crossword.bounding_box().area();
            if area < min_area && !min_areas.contains(&area) {
                min_areas.remove(&min_area);
                min_areas.insert(area);
                self.min_area.set(min_areas.last().unwrap());
            }
        }
        true
    }
}
