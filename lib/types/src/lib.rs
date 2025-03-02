pub const BOARD_WIDTH: usize = 7;
pub const BOARD_HEIGHT: usize = 6;
pub const CONSECUTIVE_COUNT_TO_WIN: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    A,
    B,
}

pub type Board = [[Option<Player>; BOARD_WIDTH]; BOARD_HEIGHT];
