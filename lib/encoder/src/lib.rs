use conn4_types::{Player, State};

pub fn encode(state: State) -> u64 {
    let mut encoded: u64 = match state.turn {
        Player::A => 0,
        Player::B => 1 << 63, // First (most significant) bit for turn
    };

    for col_idx in 0..7 {
        let mut count: u64 = 0;
        let mut player_b_bits: u64 = 0;

        // Encode from bottom to top
        for row_idx in (0..6).rev() {
            if let Some(player) = state.board[row_idx][col_idx] {
                if player == Player::B {
                    player_b_bits |= 1 << count; // Store Player.B presence
                }
                count += 1;
            }
        }

        let col_bits = (count << 6) | player_b_bits;
        let shift_amount = 1 + col_idx * 9; // Each column uses 9 bits
        encoded |= (col_bits << (64 - (shift_amount + 9))); // Align to MSB
    }

    encoded
}

pub fn decode(encoded: u64) -> State {
    let turn = if encoded & (1 << 63) != 0 {
        Player::B
    } else {
        Player::A
    };
    let mut board = [[None; 7]; 6];

    for col_idx in 0..7 {
        let shift_amount = 1 + col_idx * 9;
        let col_bits = (encoded >> (64 - (shift_amount + 9))) & 0x1FF;
        let count = (col_bits >> 6) as usize;
        let player_b_bits = col_bits & 0x3F;

        for i in 0..count {
            let is_player_b = (player_b_bits >> i) & 1 != 0;
            board[5 - i][col_idx] = Some(if is_player_b { Player::B } else { Player::A });
            // Bottom-up
        }
    }

    State { turn, board }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn generate_random_board(rng: &mut StdRng) -> State {
        let mut board = [[None; 7]; 6];

        for col_idx in 0..7 {
            // Randomly choose how many cells in the column are "filled"
            let height = rng.random_range(0..=6);
            // Fill from the bottom row upward
            for offset in 0..height {
                let row_idx = 5 - offset;
                board[row_idx][col_idx] = Some(if rng.random_bool(0.5) {
                    Player::A
                } else {
                    Player::B
                });
            }
        }

        let turn = if rng.random_bool(0.5) {
            Player::A
        } else {
            Player::B
        };

        State { turn, board }
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
