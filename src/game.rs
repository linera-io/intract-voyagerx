use crate::{gen_range, Direction, ROW_MASK};
use lazy_static::lazy_static;
use std::ops::Add;
include!("../moves_data.rs");

/// Struct that contains all available moves per row for up, down, right and left.
/// Also stores the score for a given row.
///
/// Moves are stored as power values for tiles.
/// if a power value is `> 0`, print the tile value using `2 << tile` where tile is any 4-bit
/// "nybble" otherwise print a `0` instead.
pub struct Moves {
    pub left: &'static [u64; 65536],
    pub right: &'static [u64; 65536],
    pub down: &'static [u64; 65536],
    pub up: &'static [u64; 65536],
    pub scores: &'static [u64; 65536],
}

lazy_static! {
    /// Constructs a new `tfe::Moves`.
    ///
    /// `Moves` stores `right`, `left`, `up`, and `down` moves per row.
    ///  e.g. left: `0x0011 -> 0x2000` and right: `0x0011 -> 0x0002`.
    ///
    ///  Also stores the `scores` per row.
    ///  The score of a row is the sum of the tile and all intermediate tile merges.
    ///  e.g. row `0x0002` has a score of `4` and row `0x0003` has a score of `16`.
    static ref MOVES: Moves = {
        Moves {
            left: &LEFT_MOVES,
            right: &RIGHT_MOVES,
            down: &DOWN_MOVES,
            up: &UP_MOVES,
            scores: &SCORES,
        }
    };
}

/// Struct used to play a single game of 2048.
///
/// `tfe::Game` uses a single `u64` as board value.
/// The board itself is divided into rows (x4 16 bit "row" per "board") which are
/// divided into tiles (4x 4 bit "nybbles" per "row").
///
/// All manipulations are done using bit-shifts and a precomputed table of moves and scores.
/// Every move is stored as four lookups total, one for each row. The result of XOR'ing each row
/// back into the board at the right position is the output board.
pub struct Game {
    pub board: u64,
    pub seed: u16,
}
impl Game {
    /// Constructs a new `tfe::Game`.
    ///
    /// `Game` stores a board internally as a `u64`.
    ///
    /// # Examples
    ///
    /// Simple example:
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let mut game = Game::new();
    /// # println!("{:016x}", game.board);
    /// ```
    ///
    /// Accessing board value:
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let mut game = Game::new();
    /// println!("{:016x}", game.board);
    /// ```
    pub fn new(seed: u16) -> Self {
        let mut game = Game {
            board: 0x0000_0000_0000_0000_u64,
            seed,
        };

        game.board |= Self::spawn_tile(game.board, game.seed);
        game.board |= Self::spawn_tile(game.board, game.seed + 1);

        game
    }

    /// Returns `board` moved in given `direction`.
    ///
    /// - When `Direction::Left`, return board moved left
    /// - When `Direction::Right`, return board moved right
    /// - When `Direction::Down`, return board moved down
    /// - When `Direction::Up`, return board moved up
    ///
    /// # Examples
    ///
    /// Simple example:
    ///
    /// ```
    /// use tfe::{Game, Direction};
    ///
    /// let board = 0x0000_0000_0022_1100;
    /// let moved = Game::execute(board, &[Direction::Left]);
    ///
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |  =>  | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 4 | 4 |      | 8 | 0 | 0 | 0 |
    /// // | 2 | 2 | 0 | 0 |      | 4 | 0 | 0 | 0 |
    ///
    /// assert_eq!(board, 0x0000_0000_0022_1100);
    /// assert_eq!(moved, 0x0000_0000_3000_2000);
    /// ```
    pub fn execute(&mut self, direction: Direction) -> u64 {
        let mut current_board = self.board;
        current_board = match direction {
            Direction::Left => Self::move_left(current_board),
            Direction::Right => Self::move_right(current_board),
            Direction::Down => Self::move_down(current_board),
            Direction::Up => Self::move_up(current_board),
        };

        if current_board != self.board {
            current_board = current_board | Self::spawn_tile(current_board, self.seed)
        }

        current_board
    }

    /// Converts a 64-bit board representation to a 4x4 matrix of u16 values.
    ///
    /// This function takes a u64 board representation where each 4 bits represent
    /// a tile value, and converts it into a 2D array (matrix) of u16 values.
    ///
    /// # Arguments
    ///
    /// * `board` - A u64 representing the game board, where each 4 bits encode a tile.
    ///
    /// # Returns
    ///
    /// A 4x4 array of u16 values, where each value represents the actual tile value.
    /// For example, a value of 3 represents 2^3 = 8.
    ///
    /// # Example
    ///
    /// ```
    /// let board = 0x0000_0000_0022_1100; // represents a board with 1, 1, 2, 2 in the bottom two rows
    /// let matrix = Game::convert_to_matrix(board);
    /// assert_eq!(matrix, [
    ///     [0, 0, 0, 0],
    ///     [0, 0, 0, 0],
    ///     [0, 0, 2, 2],
    ///     [1, 1, 0, 0]
    /// ]);
    /// ```
    pub fn convert_to_matrix(board: u64) -> [[u16; 4]; 4] {
        let mut matrix = [[0u16; 4]; 4];
        for i in 0..16 {
            let value = ((board >> (i * 4)) & 0xF) as u8;
            // Correct the indexing to avoid swapping left and right
            matrix[3 - (i / 4)][3 - (i % 4)] = if value > 0 { value.into() } else { 0 };
        }
        matrix
    }

    /// Determines if the game has ended.
    ///
    /// The game is considered ended if:
    /// 1. Any tile on the board has reached the value of 2048.
    /// 2. No moves in any direction (left, right, up, down) result in a change in the board.
    ///
    /// # Arguments
    ///
    /// * `board` - A `u64` representing the current state of the game board.
    ///
    /// # Returns
    ///
    /// * `true` if the game is ended, either by reaching 2048 or having no possible moves left.
    /// * `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let board = 0x0000_0000_0000_0B00; // A board with a tile value of 2048
    /// assert!(Game::is_ended(board)); // Game should be ended
    ///
    /// let board = 0x0000_0000_0000_0000; // An empty board
    /// assert!(!Game::is_ended(board)); // Game should not be ended
    /// ```
    pub fn is_ended(board: u64) -> bool {
        // Check if any tile has reached 2048
        for i in 0..16 {
            let tile_value = (board >> (i * 4)) & 0xF;
            if tile_value == 11 {
                // 2^11 = 2048
                return true;
            }
        }

        // Check if any move changes the board
        let left = Self::move_left(board);
        let right = Self::move_right(board);
        let up = Self::move_up(board);
        let down = Self::move_down(board);

        if board == left && board == right && board == up && board == down {
            return true;
        }

        false
    }

    /// Returns a transposed board where rows are transformed into columns and vice versa.
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// // | F | E | D | C |       | F | B | 7 | 3 |
    /// // | B | A | 9 | 8 |   =>  | E | A | 6 | 2 |
    /// // | 7 | 6 | 5 | 4 |       | D | 9 | 5 | 1 |
    /// // | 3 | 2 | 1 | 0 |       | C | 8 | 4 | 0 |
    ///
    /// assert_eq!(Game::transpose(0xFEDC_BA98_7654_3210), 0xFB73_EA62_D951_C840);
    /// ```
    pub fn transpose(board: u64) -> u64 {
        let a1 = board & 0xF0F0_0F0F_F0F0_0F0F_u64;
        let a2 = board & 0x0000_F0F0_0000_F0F0_u64;
        let a3 = board & 0x0F0F_0000_0F0F_0000_u64;

        let a = a1 | (a2 << 12) | (a3 >> 12);

        let b1 = a & 0xFF00_FF00_00FF_00FF_u64;
        let b2 = a & 0x00FF_00FF_0000_0000_u64;
        let b3 = a & 0x0000_0000_FF00_FF00_u64;

        b1 | (b2 >> 24) | (b3 << 24)
    }

    /// Returns a `u64` board moved up.
    /// This is the same as calling `Game::execute(board, &Direction::Up)`;
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let board  = 0x0000_0000_0000_0011_u64;
    /// let result = Game::move_up(board);
    ///
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 1 | 1 |
    /// // | 0 | 0 | 0 | 0 |  =>  | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 1 | 1 |      | 0 | 0 | 0 | 0 |
    ///
    /// assert_eq!(result, 0x0011_0000_0000_0000);
    /// ```
    pub fn move_up(board: u64) -> u64 {
        let mut result = board;
        let transposed = Self::transpose(board);

        result ^= MOVES.up[((transposed) & ROW_MASK) as usize];
        result ^= MOVES.up[((transposed >> 16) & ROW_MASK) as usize] << 4;
        result ^= MOVES.up[((transposed >> 32) & ROW_MASK) as usize] << 8;
        result ^= MOVES.up[((transposed >> 48) & ROW_MASK) as usize] << 12;

        result
    }

    /// Returns a `u64` board moved down.
    /// This is the same as calling `Game::execute(board, &Direction::Down)`;
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let board  = 0x0011_0000_0000_0011_u64;
    /// let result = Game::move_down(board);
    ///
    /// // | 0 | 0 | 1 | 1 |      | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |  =>  | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 1 | 1 |      | 0 | 0 | 2 | 2 |
    ///
    /// assert_eq!(result, 0x0000_0000_0000_0022);
    /// ```
    pub fn move_down(board: u64) -> u64 {
        let mut result = board;
        let transposed = Self::transpose(board);

        result ^= MOVES.down[((transposed) & ROW_MASK) as usize];
        result ^= MOVES.down[((transposed >> 16) & ROW_MASK) as usize] << 4;
        result ^= MOVES.down[((transposed >> 32) & ROW_MASK) as usize] << 8;
        result ^= MOVES.down[((transposed >> 48) & ROW_MASK) as usize] << 12;

        result
    }

    /// Returns a `u64` board moved right.
    /// This is the same as calling `Game::execute(board, &Direction::Right)`;
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let board  = 0x0000_0000_0000_2211_u64;
    /// let result = Game::move_right(board);
    ///
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |  =>  | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 2 | 2 | 1 | 1 |      | 0 | 0 | 3 | 2 |
    ///
    /// assert_eq!(result, 0x0000_0000_0000_0032);
    /// ```
    pub fn move_right(board: u64) -> u64 {
        let mut result = board;

        result ^= MOVES.right[((board) & ROW_MASK) as usize];
        result ^= MOVES.right[((board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.right[((board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.right[((board >> 48) & ROW_MASK) as usize] << 48;

        result
    }

    /// Returns a `u64` board moved left.
    /// This is the same as calling `Game::execute(board, &Direction::Left)`;
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let board  = 0x0000_0000_0000_2211_u64;
    /// let result = Game::move_left(board);
    ///
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |  =>  | 0 | 0 | 0 | 0 |
    /// // | 0 | 0 | 0 | 0 |      | 0 | 0 | 0 | 0 |
    /// // | 2 | 2 | 1 | 1 |      | 3 | 2 | 0 | 0 |
    ///
    /// assert_eq!(result, 0x0000_0000_0000_3200);
    /// ```
    pub fn move_left(board: u64) -> u64 {
        let mut result: u64 = board;

        result ^= MOVES.left[((board) & ROW_MASK) as usize];
        result ^= MOVES.left[((board >> 16) & ROW_MASK) as usize] << 16;
        result ^= MOVES.left[((board >> 32) & ROW_MASK) as usize] << 32;
        result ^= MOVES.left[((board >> 48) & ROW_MASK) as usize] << 48;

        result
    }

    /// Returns the count of tiles with a value of `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tfe::Game;
    ///
    /// let board  = 0x0000_0000_0000_2211_u64;
    /// let result = Game::count_empty(board);
    ///
    /// assert_eq!(result, 12);
    /// ```
    pub fn count_empty(board: u64) -> u16 {
        let mut empty = 0;

        for i in 0..16 {
            if ((board >> (i * 4)) & 0xF) == 0 {
                empty += 1
            }
        }

        empty
    }

    /// Returns the sum of 4 lookups in `table` for each "row" in `board`.
    pub fn table_helper<T: Clone + Add<Output = T>>(board: u64, table: &[T]) -> T {
        table[((board) & ROW_MASK) as usize].clone()
            + table[((board >> 16) & ROW_MASK) as usize].clone()
            + table[((board >> 32) & ROW_MASK) as usize].clone()
            + table[((board >> 48) & ROW_MASK) as usize].clone()
    }

    /// Returns the score of a given `board`.
    /// The score of a single tile is the sum of the tile value and all intermediate merged tiles.
    pub fn score(board: u64) -> u64 {
        Self::table_helper(board, MOVES.scores)
    }

    /// Returns a `2` with 90% chance and `4` with 10% chance.
    pub fn tile(seed: u16) -> u64 {
        if gen_range(&seed.to_string(), 0, 10) == 10 {
            2
        } else {
            1
        }
    }

    /// Returns a `1` shifted to the position of any `0` bit in `board` randomly.
    pub fn spawn_tile(board: u64, seed: u16) -> u64 {
        let mut tmp = board;
        let mut idx = gen_range(&seed.to_string(), 0, Self::count_empty(board));
        let mut t = Self::tile(seed);

        loop {
            while (tmp & 0xF) != 0 {
                tmp >>= 4;
                t <<= 4;
            }

            if idx == 0 {
                break;
            } else {
                idx -= 1
            }

            tmp >>= 4;
            t <<= 4
        }

        t
    }
}
