use game2048::Moves;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut left_moves = vec![0u64; 65536];
    let mut right_moves = vec![0u64; 65536];
    let mut up_moves = vec![0u64; 65536];
    let mut down_moves = vec![0u64; 65536];
    let mut scores = vec![0u64; 65536];

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
        // generates 4 output moves for up, down, left and right by transposing and reversing
        // this result.
        while i < 3 {
            // initial counter for the cell next to the current one (j)
            let mut j = i + 1;

            // find the next non-zero cell index
            while j < 4 {
                if line[j] != 0 {
                    break;
                };
                j += 1;
            }

            // if j is out of bounds (> 3), all other cells are empty and we are done looping
            if j == 4 {
                break;
            };

            // this is the part responsible for skipping empty (0 value) cells
            // if the current cell is zero, shift the next non-zero cell to position i
            // and retry this entry until line[i] becomes non-zero
            if line[i] == 0 {
                line[i] = line[j];
                line[j] = 0;
                continue;

            // otherwise, if the current cell and next cell are the same, merge them
            } else if line[i] == line[j] {
                if line[i] != 0xF {
                    line[i] += 1
                };
                line[j] = 0;
            }

            // finally, move to the next (or current, if i was 0) row
            i += 1;
        }

        // put the new row after merging back together into a "merged" row
        let result = (line[0]) | (line[1] << 4) | (line[2] << 8) | (line[3] << 12);

        // right and down use normal row and result variables.
        // for left and up, we create a reverse of the row and result.
        let rev_row =
            (row >> 12) & 0x000F | (row >> 4) & 0x00F0 | (row << 4) & 0x0F00 | (row << 12) & 0xF000;
        let rev_res = (result >> 12) & 0x000F
            | (result >> 4) & 0x00F0
            | (result << 4) & 0x0F00
            | (result << 12) & 0xF000;

        // results are keyed by row / reverse row index.
        let row_idx = row as usize;
        let rev_idx = rev_row as usize;

        right_moves[row_idx] = row ^ result;
        left_moves[rev_idx] = rev_row ^ rev_res;
        up_moves[rev_idx] = Moves::column_from(rev_row) ^ Moves::column_from(rev_res);
        down_moves[row_idx] = Moves::column_from(row) ^ Moves::column_from(result);
    }

    let mut file = File::create("moves_data.rs").unwrap();
    writeln!(
        file,
        "pub const LEFT_MOVES: [u64; 65536] = {:?};",
        left_moves
    )
    .unwrap();
    writeln!(
        file,
        "pub const RIGHT_MOVES: [u64; 65536] = {:?};",
        right_moves
    )
    .unwrap();
    writeln!(file, "pub const UP_MOVES: [u64; 65536] = {:?};", up_moves).unwrap();
    writeln!(
        file,
        "pub const DOWN_MOVES: [u64; 65536] = {:?};",
        down_moves
    )
    .unwrap();
    writeln!(file, "pub const SCORES: [u64; 65536] = {:?};", scores).unwrap();
}
