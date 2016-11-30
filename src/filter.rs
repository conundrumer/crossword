use std::i16::MAX;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::cell::Cell;
use std::cell::RefCell;

use placement::Position;
use bounding_box::BoundingBox;
use word_placements::WordPlacements;
use crossword::Crossword;
#[derive(Debug)]
struct OverlapAreas {
    overlap_areas: RefCell<Vec<RefCell<BTreeSet<i16>>>>,
    max_area: Cell<(i8, i16)>,
    pub num_areas: usize
}
impl OverlapAreas {
    fn new(num_areas: usize) -> OverlapAreas {
        OverlapAreas {
            overlap_areas: RefCell::new(vec![]),
            max_area: Cell::new((0, MAX)),
            num_areas: num_areas
        }
    }
    fn get_max_area(&self) -> i16 {
        self.max_area.get().1
    }
    fn reset_max_area(&self, overlaps: i8, area: i16) {
        let max_area = self.overlap_areas.borrow().iter().enumerate().fold(
            (overlaps, area),
            |(prev_overlaps, prev_area), (overlaps, cell_areas)| {
                if let Some(area) = cell_areas.borrow().last() {
                    if area > prev_area {
                        return (overlaps as i8, area)
                    }
                }
                (prev_overlaps, prev_area)
            }
        );
        self.max_area.set(max_area);
    }
    fn extend_if_needed(&self, overlaps: i8) {
        let overlaps = overlaps as usize;
        let overlap_areas_len = self.overlap_areas.borrow().len();
        if overlaps >= overlap_areas_len {
            let mut overlap_areas = self.overlap_areas.borrow_mut();
            overlap_areas.extend((overlap_areas_len..overlaps + 1).map(|_| RefCell::new(BTreeSet::new())));
        }
    }
    fn filter_by_area(&self, overlaps: i8, area: i16) -> bool {
        let mut should_reset = false;
        {
            self.extend_if_needed(overlaps);
            let overlap_areas =  self.overlap_areas.borrow();
            let mut areas = overlap_areas[overlaps as usize].borrow_mut();
            if let Some(last_area) = areas.last() {
                if area > last_area {
                    return false;
                }
                if area < last_area && !areas.contains(&area) {
                    if areas.len() == self.num_areas {
                        areas.remove(&last_area);
                    }
                    areas.insert(area);
                    let (max_area_overlaps, max_area) = self.max_area.get();
                    should_reset = overlaps == max_area_overlaps && area < max_area;
                }
            } else {
                areas.insert(area);
                should_reset = true;
            }
        }
        if should_reset {
            self.reset_max_area(overlaps, area);
        }
        true
    }
}

#[derive(Debug)]
pub struct Filter {
    seen: RefCell<HashSet<WordPlacements>>,
    overlap_areas: OverlapAreas,
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
        Filter {
            seen: RefCell::new(HashSet::new()),
            overlap_areas: OverlapAreas::new(num_areas),
            has_min_areas: num_areas > 0
        }
    }
    pub fn num_areas(&self) -> usize {
        self.overlap_areas.num_areas
    }

    pub fn by_area(&self, word_len: usize, next_pos: Position, bb: BoundingBox) -> bool {
        if !self.has_min_areas {
            return true
        }
        let area = bb.combine_word_pos(word_len, next_pos).area();
        area <= self.overlap_areas.get_max_area()
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
            let area = crossword.bounding_box().area();
            let overlaps = crossword.num_overlaps();

            let out = self.overlap_areas.filter_by_area(overlaps, area);
            return out
        }
        true
    }
}
