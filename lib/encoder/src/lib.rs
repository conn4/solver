use conn4_types::{Board, Player};

pub fn encode(board: Board) -> u64 {
    let mut result: u64 = 0;

    for col in 0..7 {
        let mut column = 1;
        for row in 0..6 {
            if let Some(player) = board[row][col] {
                column = (column << 1) | if player == Player::A { 0 } else { 1 };
            } else {
                break;
            }
        }
        result |= column << (col * 7);
    }

    result
}

pub fn decode(encoded: u64) -> Board {
    let mut result = [[None; 7]; 6];

    for col in 0..7 {
        let mut column = (encoded >> (col * 7)) & 0b1111111;
        let mut rev = Vec::new();
        while column > 1 {
            rev.push(if column & 1 == 0 {
                Player::A
            } else {
                Player::B
            });
            column >>= 1;
        }
        rev.reverse();
        for row in 0..rev.len() {
            result[row][col] = Some(rev[row]);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn generate_random_board(rng: &mut StdRng) -> Board {
        let mut board = [[None; 7]; 6];

        for col_idx in 0..7 {
            // Randomly choose how many cells in the column are "filled"
            let height = rng.random_range(0..=6);
            // Fill from the bottom row upward
            for row_idx in 0..height {
                board[row_idx][col_idx] = Some(if rng.random_bool(0.5) {
                    Player::A
                } else {
                    Player::B
                });
            }
        }

        board
    }

    #[test]
    fn test_encoding_decoding() {
        let mut rng = StdRng::seed_from_u64(42); // Fixed seed for reproducibility

        for _ in 0..1000 {
            let original_state = generate_random_board(&mut rng);
            let encoded = encode(original_state);
            let decoded_state = decode(encoded);

            assert_eq!(decoded_state, original_state, "State mismatch!");
        }
    }
}
