use placement::{ Position, Direction };
use placement::Direction::{ Horizontal, Vertical };

type GridCellPos = (GridCell, (i8, i8));
fn block_dir(dir: Direction, row: i8, col: i8) -> impl Iterator<Item=GridCellPos> {
    Some((Block(Some(dir)), (row, col))).into_iter()
}
fn block_none(row: i8, col: i8) -> impl Iterator<Item=GridCellPos> {
    Some((Block(None), (row, col))).into_iter()
}
fn letter(c: char, dir: Direction, row: i8, col: i8) -> impl Iterator<Item=GridCellPos> {
    Some((Letter(c, Some(dir)), (row, col))).into_iter()
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GridCell {
    Empty,
    Block(Option<Direction>),
    Letter(char, Option<Direction>),
    Collision
}
use self::GridCell::*;
impl GridCell {

    pub fn from_word<'a>(word: &'a str, pos: Position) -> impl Iterator<Item=GridCellPos> + 'a {
        let letter_iter = word.chars().enumerate()
            .map(move |(j, c)| {
                (pos.letter_pos(j as i8), c)
            })
            .flat_map(|(pos, c)| {
                let ((row1, col1), (row2, col2)) = match pos.dir {
                    Horizontal => ((pos.row - 1, pos.col), (pos.row + 1, pos.col)),
                    Vertical => ((pos.row, pos.col - 1), (pos.row, pos.col + 1))
                };
                let cell1 = block_dir(pos.dir, row1, col1);
                let cell2 = letter(c, pos.dir, pos.row, pos.col);
                let cell3 = block_dir(pos.dir, row2, col2);
                cell1.chain(cell2).chain(cell3)
            });
        let start_pos = pos.letter_pos(-1);
        let end_pos = pos.letter_pos(word.len() as i8);
        let start_cell = block_none(start_pos.row, start_pos.col);
        let end_cell = block_none(end_pos.row, end_pos.col);
        start_cell.chain(letter_iter).chain(end_cell)
    }

    pub fn get_next(self, cell: GridCell) -> GridCell {
        // cell: Block(Some(_)), Block(None), Letter(_, Some(_))
        // old_cell: Empty, Block(Some(_)), Block(None), Letter(_, Some(_)), Letter(_, None), Collision
        match (cell, self) {
            (_, Empty) => {
                cell
            },
            (Block(Some(o1)), Block(Some(o2))) if o1 == o2 => {
                Block(Some(o1))
            },
            (Block(_), Block(_)) => {
                Block(None)
            },
            (Letter(c1, Some(o1)), Letter(c2, Some(o2))) if c1 == c2 && o1 != o2 => {
                Letter(c1, None)
            },
            (Letter(c, opt_o1 @ _), Block(opt_o2 @ _)) | (Block(opt_o2 @ _), Letter(c, opt_o1 @ _)) => {
                match (opt_o1, opt_o2) {
                    (Some(o1), Some(o2)) if o1 != o2 => Letter(c, None),
                    (None, _) => Letter(c, None),
                    _ => Collision
                }
            },
            (_, _) => match cell {
                Block(_) | Letter(_, Some(_)) => Collision,
                _ => unreachable!()
            }
        }
    }
}
