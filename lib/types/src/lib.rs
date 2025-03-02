#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    A,
    B,
}

pub type Board = [[Option<Player>; 7]; 6];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct State {
    pub turn: Player,
    pub board: Board,
}
