use placement::Position;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct WordPlacements(Vec<Option<Position>>);

impl WordPlacements {
    pub fn new(n: usize) -> WordPlacements {
        WordPlacements(vec![None; n])
    }
    // immutable set: returns a new copy
    pub fn set(&self, word_index: usize, pos: Position) -> WordPlacements {
        let mut next_self = self.clone();
        next_self.0[word_index] = Some(pos);
        next_self
    }
}

use std::fmt::{Display, Formatter, Result};
impl Display for WordPlacements {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (i, &opt_pos) in self.0.iter().enumerate() {
            if let Some(pos) = opt_pos {
                write!(f, "{}", pos)?;
                if i < self.0.len() - 1 {
                    write!(f, ",")?;
                }
            }
        }
        write!(f, "")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use placement::Position;
    use placement::Direction::{ Horizontal, Vertical };

    #[test]
    fn new() {
        let wp1 = WordPlacements(vec![None]);
        assert_eq!(wp1, WordPlacements::new(1));

        let wp2 = WordPlacements(vec![None, None]);
        assert_eq!(wp2, WordPlacements::new(2));
    }
    #[test]
    fn set() {
        let pos = Position { row: 1, col: -1, dir: Vertical };
        let wp = WordPlacements(vec![
            None,
            Some(pos)
        ]);
        assert_eq!(wp, WordPlacements::new(2).set(1, pos));
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
