use placement::{ Position, GridIndex, Orientation };
use placement::Orientation::{ Horizontal, Vertical };

#[derive(Debug)]
pub struct Word {
    pub letters: &'static str,
    pub pos: Position,
    pub orientation: Orientation
}
impl Word {
    pub fn len(&self) -> GridIndex {
        self.letters.len() as GridIndex
    }
    pub fn letter_pos(&self, i: GridIndex) -> Position {
        match self.orientation {
            Horizontal => Position {
                row: self.pos.row,
                col: self.pos.col + i
            },
            Vertical => Position {
                row: self.pos.row + i,
                col: self.pos.col
            }
        }
    }
    pub fn last_pos(&self) -> Position {
        self.letter_pos((self.len() - 1) as GridIndex)
    }
}
