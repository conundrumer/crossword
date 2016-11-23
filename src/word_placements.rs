use placement::Position;
use placement::Direction::Horizontal;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct WordPlacements(pub Vec<Option<Position>>);

impl WordPlacements {
    pub fn new(n: usize) -> WordPlacements {
        let mut positions = vec![None; n];
        positions[0] = Some(Position { row: 0, col: 0, dir: Horizontal });
        WordPlacements(positions)
    }
    // immutable set: returns a new copy
    pub fn set(&self, word_index: usize, pos: Position) -> WordPlacements {
        let mut next_self = self.clone();
        next_self.0[word_index] = Some(pos);
        next_self
    }
    pub fn index_positions<'a>(&'a self) -> impl Iterator<Item=(usize, Position)> + 'a {
        self.0.iter().enumerate().flat_map(|(word_index, opt_pos)| opt_pos.map(|pos| (word_index, pos)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use placement::Position;
    use placement::Direction::{ Horizontal, Vertical };

    #[test]
    fn new() {
        let wp1 = WordPlacements(vec![Some(Position { row: 0, col: 0, dir: Horizontal })]);
        assert_eq!(wp1, WordPlacements::new(1));

        let wp2 = WordPlacements(vec![Some(Position { row: 0, col: 0, dir: Horizontal }), None]);
        assert_eq!(wp2, WordPlacements::new(2));
    }
    #[test]
    fn set() {
        let pos = Position { row: 1, col: -1, dir: Vertical };
        let wp = WordPlacements(vec![
            Some(Position { row: 0, col: 0, dir: Horizontal }),
            Some(pos)
        ]);
        assert_eq!(wp, WordPlacements::new(2).set(1, pos));
    }
    #[test]
    fn index_positions() {
        let pos = Position { row: 1, col: -1, dir: Vertical };

        let wp = WordPlacements::new(3).set(2, pos);
        let iter = wp.index_positions();
        let vec: Vec<_> = iter.collect();
        assert_eq!(vec![(0, Position { row: 0, col: 0, dir: Horizontal }), (2, pos)], vec);
    }

    #[test]
    fn hash() {
        use std::collections::HashSet;
        let wp1 = WordPlacements(vec![Some(Position { row: 0, col: 0, dir: Horizontal })]);
        let wp2 = WordPlacements(vec![Some(Position { row: 0, col: 0, dir: Horizontal })]);
        let mut set = HashSet::new();
        set.insert(wp1.clone());
        assert!(set.contains(&wp2));
    }
}
