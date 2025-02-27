#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    A,
    B,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct State {
    pub turn: Player,
    pub board: [[Option<Player>; 7]; 6],
}
