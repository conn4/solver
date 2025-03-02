use conn4_types::{Board, Player, BOARD_HEIGHT, BOARD_WIDTH, CONSECUTIVE_COUNT_TO_WIN};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Result {
    Win,
    Draw,
    Lose,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    End(Result),
    Ongoing(Player),
}

pub fn result(board: Board) -> Option<Result> {
    for row in 0..BOARD_HEIGHT {
        for col in 0..BOARD_WIDTH {
            if let Some(player) = board[row][col] {
                let mut win = Some(player);
                for i in 1..CONSECUTIVE_COUNT_TO_WIN {
                    if row + i >= BOARD_HEIGHT || board[row + i][col] != board[row][col] {
                        win = None;
                        break;
                    }
                }
                if let Some(win) = win {
                    return Some(if win == Player::A {
                        Result::Win
                    } else {
                        Result::Lose
                    });
                }

                let mut win = Some(player);
                for i in 1..CONSECUTIVE_COUNT_TO_WIN {
                    if col + i >= BOARD_WIDTH || board[row][col + i] != board[row][col] {
                        win = None;
                        break;
                    }
                }
                if let Some(win) = win {
                    return Some(if win == Player::A {
                        Result::Win
                    } else {
                        Result::Lose
                    });
                }

                let mut win = Some(player);
                for i in 1..CONSECUTIVE_COUNT_TO_WIN {
                    if col + i >= BOARD_WIDTH
                        || row + i >= BOARD_HEIGHT
                        || board[row + i][col + i] != board[row][col]
                    {
                        win = None;
                        break;
                    }
                }
                if let Some(win) = win {
                    return Some(if win == Player::A {
                        Result::Win
                    } else {
                        Result::Lose
                    });
                }

                let mut win = Some(player);
                for i in 1..CONSECUTIVE_COUNT_TO_WIN {
                    if col + i >= BOARD_WIDTH
                        || row < i
                        || board[row - i][col + i] != board[row][col]
                    {
                        win = None;
                        break;
                    }
                }
                if let Some(win) = win {
                    return Some(if win == Player::A {
                        Result::Win
                    } else {
                        Result::Lose
                    });
                }
            }
        }
    }

    for col in 0..BOARD_WIDTH {
        if board[BOARD_HEIGHT - 1][col] == None {
            return None;
        }
    }

    Some(Result::Draw)
}

pub fn state(board: Board) -> State {
    if let Some(result) = result(board) {
        return State::End(result);
    }

    let count_a = board.iter().fold(0, |acc, curr| {
        acc + curr.iter().fold(0, |acc, curr| {
            acc + if *curr == Some(Player::A) { 1 } else { 0 }
        })
    });
    let count_b = board.iter().fold(0, |acc, curr| {
        acc + curr.iter().fold(0, |acc, curr| {
            acc + if *curr == Some(Player::B) { 1 } else { 0 }
        })
    });

    if count_b > count_a || count_a > count_b + 1 {
        panic!("count mismatch");
    }

    State::Ongoing(if count_a == count_b {
        Player::A
    } else {
        Player::B
    })
}

pub fn positions(board: Board) -> Vec<usize> {
    let mut result = Vec::new();

    for col in 0..BOARD_WIDTH {
        if board[BOARD_HEIGHT - 1][col].is_none() {
            result.push(col);
        }
    }

    result
}

pub fn play(board: Board, position: usize) -> Board {
    let current_player = match state(board) {
        State::End(_) => panic!("game is already end"),
        State::Ongoing(player) => player,
    };

    for row in 0..BOARD_HEIGHT {
        if board[row][position].is_none() {
            let mut result = board;
            result[row][position] = Some(current_player);
            return result;
        }
    }

    panic!("impossible position given");
}
