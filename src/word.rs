use placement::{ Position, GridIndex };
use placement::Direction::{ Horizontal, Vertical };

#[derive(Debug, Clone)]
pub struct WordPosition<'a> {
    pub word: &'a str,
    pub pos: Position
}
impl<'a> WordPosition<'a> {
    pub fn chars(&self) -> Vec<(char, Position)> {
        self.word.chars().enumerate().map(|(i, c)| (c, self.letter_pos(i as GridIndex))).collect()
    }
    pub fn len(&self) -> GridIndex {
        self.word.len() as GridIndex
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
