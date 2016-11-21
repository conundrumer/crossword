use placement::{ Position, GridIndex };
use placement::Direction::{ Horizontal, Vertical };

#[derive(Debug, Clone)]
pub struct Word<'a> {
    pub letters: &'a str,
    pub pos: Position
}
impl<'a> Word<'a> {
    pub fn chars(&self) -> Vec<(char, Position)> {
        self.letters.chars().enumerate().map(|(i, c)| (c, self.letter_pos(i as GridIndex))).collect()
    }
    pub fn len(&self) -> GridIndex {
        self.letters.len() as GridIndex
    }
    pub fn letter_pos(&self, i: GridIndex) -> Position {
        match self.pos.dir {
            Horizontal => Position {
                row: self.pos.row,
                col: self.pos.col + i,
                dir: self.pos.dir
            },
            Vertical => Position {
                row: self.pos.row + i,
                col: self.pos.col,
                dir: self.pos.dir
            }
        }
    }
    pub fn last_pos(&self) -> Position {
        self.letter_pos((self.len() - 1) as GridIndex)
    }
}
