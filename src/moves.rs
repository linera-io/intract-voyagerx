/// A mask with a single section of 16 bits set to 0.
/// Used to extract a "horizontal slice" out of a 64 bit integer.
pub static ROW_MASK: u64 = 0xFFFF;

/// A `u64` mask with 4 sections each starting after the n * 16th bit.
/// Used to extract a "vertical slice" out of a 64 bit integer.
pub static COL_MASK: u64 = 0x000F_000F_000F_000F_u64;

/// Struct that contains all available moves per row for up, down, right and left.
/// Also stores the score for a given row.
///
/// Moves are stored as power values for tiles.
/// if a power value is `> 0`, print the tile value using `2 << tile` where tile is any 4-bit
/// "nybble" otherwise print a `0` instead.
pub struct Moves {
    pub left: Vec<u64>,
    pub right: Vec<u64>,
    pub down: Vec<u64>,
    pub up: Vec<u64>,
    pub scores: Vec<u64>,
}

impl Moves {
    /// Returns the 4th bit from each row in given board OR'd.
    pub fn column_from(board: u64) -> u64 {
        (board | (board << 12) | (board << 24) | (board << 36)) & COL_MASK
    }

    /// Constructs a new `Moves` instance.
    ///
    /// `Moves` stores `right`, `left`, `up`, and `down` moves per row.
    /// Also stores the `scores` per row.
    pub fn new() -> Moves {
        // initialization of move tables
        let mut left_moves = vec![0; 65536];
        let mut right_moves = vec![0; 65536];
        let mut up_moves = vec![0; 65536];
        let mut down_moves = vec![0; 65536];
        let mut scores = vec![0; 65536];

        for row in 0..65536 {
            // break row into cells
            let mut line = [
                (row) & 0xF,
                (row >> 4) & 0xF,
                (row >> 8) & 0xF,
                (row >> 12) & 0xF,
            ];

            // calculate score for given row
            let mut s = 0;

            for &tile in &line {
                if tile > 1 {
                    s += (tile - 1) * (2 << tile)
                }
            }

            scores[row as usize] = s;

            let mut i = 0;

            // perform a move to the left using current {row} as board
            while i < 3 {
                let mut j = i + 1;

                while j < 4 {
                    if line[j] != 0 {
                        break;
                    };
                    j += 1;
                }

                if j == 4 {
                    break;
                };

                if line[i] == 0 {
                    line[i] = line[j];
                    line[j] = 0;
                    continue;
                } else if line[i] == line[j] {
                    if line[i] != 0xF {
                        line[i] += 1
                    };
                    line[j] = 0;
                }

                i += 1;
            }

            let result = (line[0]) | (line[1] << 4) | (line[2] << 8) | (line[3] << 12);

            let rev_row = (row >> 12) & 0x000F
                | (row >> 4) & 0x00F0
                | (row << 4) & 0x0F00
                | (row << 12) & 0xF000;
            let rev_res = (result >> 12) & 0x000F
                | (result >> 4) & 0x00F0
                | (result << 4) & 0x0F00
                | (result << 12) & 0xF000;

            let row_idx = row as usize;
            let rev_idx = rev_row as usize;

            right_moves[row_idx] = row ^ result;
            left_moves[rev_idx] = rev_row ^ rev_res;
            up_moves[rev_idx] = Moves::column_from(rev_row) ^ Moves::column_from(rev_res);
            down_moves[row_idx] = Moves::column_from(row) ^ Moves::column_from(result);
        }

        Moves {
            left: left_moves,
            right: right_moves,
            down: down_moves,
            up: up_moves,
            scores,
        }
    }
}

impl Default for Moves {
    fn default() -> Self {
        Moves::new()
    }
}
