/// Maze Helpers

/// 4 commonly used directions (row, col) for mazes and whatnot
pub const FOUR_DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), // up
    (1, 0),  // down
    (0, -1), // left
    (0, 1),  // right
];

/// 8 commonly used directions (row, col) for mazes & whatnot
pub const EIGHT_DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), // up-left
    (-1, 0),  // up
    (-1, 1),  // up-right
    (0, -1),  // left
    (0, 1),   // right
    (1, -1),  // down-left
    (1, 0),   // down
    (1, 1),   // down-right
];

/// Check if a position is within map edges for mazes & whatnot
pub fn is_in_map(rows: usize, cols: usize, position: (isize, isize)) -> bool {
    position.0 >= 0 && position.1 >= 0 && position.0 < rows as isize && position.1 < cols as isize
}

/// style string for indicatif::ProgressBar
pub const STYLE: &str = "{bar:40.cyan/blue} {pos:>9}/{len:9} [{eta} left] {msg}";
