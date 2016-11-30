use placement::Position;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BoundingBox {
    pub top: i8,
    pub left: i8,
    pub bottom: i8,
    pub right: i8
}
impl BoundingBox {
    pub fn new(top: i8, left: i8, bottom: i8, right: i8) -> BoundingBox {
        BoundingBox {
            top: top,
            left: left,
            bottom: bottom,
            right: right
        }
    }
    pub fn from_word_pos(word_len: usize, pos: Position) -> BoundingBox {
        let last_pos = pos.letter_pos((word_len - 1) as i8);
        BoundingBox::new(pos.row, pos.col, last_pos.row, last_pos.col)
    }
    pub fn combine(&self, other: BoundingBox) -> BoundingBox {
        use std::cmp::{min, max};
        BoundingBox::new(
            min(self.top, other.top),
            min(self.left, other.left),
            max(self.bottom, other.bottom),
            max(self.right, other.right)
        )
    }
    pub fn expand(&self) -> BoundingBox {
        BoundingBox::new(self.top - 1, self.left - 1, self.bottom + 1, self.right + 1)
    }
    pub fn contract(&self) -> BoundingBox {
        BoundingBox::new(self.top + 1, self.left + 1, self.bottom - 1, self.right - 1)
    }
    pub fn combine_word_pos(&self, word_len: usize, pos: Position) -> BoundingBox {
        self.combine(BoundingBox::from_word_pos(word_len, pos))
    }
    pub fn row_col(&self, row: i8, col: i8) -> usize {
        let row = (row - self.top) as usize;
        let col = (col - self.left) as usize;
        let width = self.width() as usize;
        width * row + col
    }
    pub fn row_col_inverse(&self, i: usize) -> (i8, i8) {
        let i = i as i16;
        let top = self.top as i16;
        let left = self.left as i16;
        let width = self.width() as i16;
        ((i / width + top) as i8, (i % width + left) as i8)
    }
    pub fn width(&self) -> i16 {
        self.right as i16 - self.left as i16 + 1
    }
    pub fn height(&self) -> i16 {
        self.bottom as i16 - self.top as i16 + 1
    }
    pub fn area(&self) -> i16 {
        self.width() * self.height()
    }
}
