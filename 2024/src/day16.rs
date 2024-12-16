use aoc;
use ndarray::Array2;
use pathfinding::directed::astar::astar_bag_collect;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    current: (usize, usize),
    previous: (usize, usize),
}

#[rustfmt::skip]
fn is_in_maze(rows: usize, cols: usize, position: (isize, isize)) -> bool {
    position.0 >= 0
    && position.1 >= 0
    && position.0 < rows as isize
    && position.1 < cols as isize
}

/// This Position object will be used to
/// 1. keep current position
/// 2. keep prior position
/// 3. compute possible successors (neighbors) & cost
/// 4. compute hueristic (distance to goal)
impl Position {
    fn heuristic(&self, goal: (usize, usize)) -> usize {
        (self.current.0 as isize - goal.0 as isize).abs() as usize
            + (self.current.1 as isize - goal.1 as isize).abs() as usize
    }

    fn successors(&self, maze: &Array2<char>) -> Vec<(Position, usize)> {
        let mut neighbors = Vec::new();

        for (drow, dcol) in aoc::DIRECTIONS {
            // calculate possible position and the cost to move there
            let new_pos = (
                self.current.0 as isize + drow,
                self.current.1 as isize + dcol,
            );
            if !is_in_maze(maze.nrows(), maze.ncols(), new_pos) {
                // skip if outside maze
                continue;
            }
            let new_pos_usize = (new_pos.0 as usize, new_pos.1 as usize);
            if new_pos_usize == self.previous {
                // skip if prior position
                continue;
            }
            if maze[new_pos_usize] == '.' || maze[new_pos_usize] == 'E' {
                // this new position is an empty space
                let mut cost = 1usize;
                let new_drow = new_pos.0 - self.current.0 as isize;
                let new_dcol = new_pos.1 - self.current.1 as isize;
                let pri_drow = self.current.0 as isize - self.previous.0 as isize;
                let pri_dcol = self.current.1 as isize - self.previous.1 as isize;
                if new_dcol != pri_dcol || new_drow != pri_drow {
                    // we need to turn to execute this position -> higher cost
                    // +1000 for turn and +1 for move
                    cost = 1001;
                }
                // this is a valid position & cost
                neighbors.push((
                    Position {
                        current: (new_pos.0 as usize, new_pos.1 as usize),
                        previous: self.current,
                    },
                    cost,
                ));
            }
        }
        return neighbors;
    }
}

/// define a __repr__() method for Position (custom debug formatting)
impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.current)
    }
}

/// Reindeer Maze
fn part(filename: &str, is_part1: bool) -> usize {
    // parse info
    let (maze, rows, cols) = aoc::read_2d_chars(filename);
    let prior: (usize, usize) = (rows - 2, 0);
    let start: (usize, usize) = (rows - 2, 1);
    let goal: (usize, usize) = (1, cols - 2);

    // compute all best paths
    let (solutions, score): (Vec<Vec<Position>>, usize) = astar_bag_collect(
        &Position {
            current: start,
            previous: prior,
        },
        |p| p.successors(&maze),
        |p| p.heuristic(goal),
        |p| p.current == goal,
    )
    .unwrap();
    // for path in &solutions {
    //     println!("Path found with score {}: {:?}", score, path);
    // }
    if is_part1 {
        return score;
    } else {
        // find all best positions in maze
        let mut best_path: Array2<bool> = Array2::from_elem((rows, cols), false);
        for path in &solutions {
            for position in path {
                best_path[position.current] = true;
            }
        }
        // sum all true values
        return best_path.iter().filter(|&&x| x).count();
    }
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 7036);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));
    assert_eq!(part(&format!("input/{:02}_train", day), false), 45);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 148 Minutes");
}
