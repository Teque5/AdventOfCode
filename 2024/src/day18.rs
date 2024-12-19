use aoc;
use ndarray::Array2;
use pathfinding::directed::astar::astar;

/// RAM Run
fn part(filename: &str, is_part1: bool, is_train: bool) -> String {
    // setup vars
    let edge: usize = if is_train { 7 } else { 71 };
    let start = (0usize, 0usize);
    let goal = (edge - 1, edge - 1);
    let mut ram: Array2<char> = Array2::from_elem((edge, edge), ' ');
    let mut img = aoc::Image::new(edge, edge);
    img.set_fontsize(2.0);
    img.set_framerate(30);

    // parse bytes
    let lines = aoc::read_lines(filename);
    let mut bytes: Vec<(usize, usize)> = Vec::new();
    for line in lines {
        let numbers = aoc::parse_numbers::<usize>(&line);
        bytes.push((numbers[1], numbers[0]))
    }

    // compute a best path if possible
    let mut solution: Vec<Position>;
    let mut cost = 0usize;

    if is_part1 {
        // fill ram
        for (bdx, byte) in bytes.clone().into_iter().enumerate() {
            ram[byte] = '#';
            if is_train && bdx == 11 {
                break;
            } else if bdx == 1023 {
                break;
            }
        }
        // show maze
        // aoc::print_2d_chars(&ram);

        // solve maze
        if let Some((mut _solution, _cost)) = astar(
            &Position { current: start },
            |p| p.successors(&ram),
            |p| p.heuristic(goal),
            |p| p.current == goal,
        ) {
            solution = _solution;
            cost = _cost;
        } else {
            println!("no solution!");
        }
        return cost.to_string();
    } else {
        let mut bad_bdx = 0usize;
        let mut prior_cost = 0usize;
        // part2 we gradually fill maze and figure out when it's not navigable
        for (bdx, byte) in bytes.clone().into_iter().enumerate() {
            // write to ram
            ram[byte] = '#';
            // show maze
            // solve maze
            if let Some((mut _solution, _cost)) = astar(
                &Position { current: start },
                |p| p.successors(&ram),
                |p| p.heuristic(goal),
                |p| p.current == goal,
            ) {
                // solution found
                prior_cost = cost;
                solution = _solution;
                cost = _cost;
            } else {
                // no solution
                bad_bdx = bdx;
                break;
            }
            // draw every 10th frame or when cost changes
            if !is_train && (bdx % 10 == 0 || prior_cost != cost) {
                img.draw_chars(&ram);
                img.fade();
                for position in solution {
                    img.draw_bool(position.current.0, position.current.1, true);
                }
                // img.draw_text(edge - 3, 0, &format!("byte {}", bdx));
                // img.draw_text(edge - 2, 0, "Teque5");
                // img.draw_text(edge - 1, 0, "Advent of Code 2024");
                img.render_frame();
            }
        }
        if !is_train {
            for _ in 0..90 {
                img.render_frame();
            }
            // img.render_webp(&"img/day18.webp");
            img.render_gif(&"img/day18.gif");
        }
        return format!("{},{}", bytes[bad_bdx].1, bytes[bad_bdx].0);
    }
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    current: (usize, usize),
}

#[rustfmt::skip]
fn is_in_ram(rows: usize, cols: usize, position: (isize, isize)) -> bool {
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
            if !is_in_ram(maze.nrows(), maze.ncols(), new_pos) {
                // skip if outside maze
                continue;
            }
            let new_pos_usize = (new_pos.0 as usize, new_pos.1 as usize);
            if maze[new_pos_usize] == ' ' {
                // this new position is an empty space
                let cost = 1usize;
                // this is a valid position
                neighbors.push((
                    Position {
                        current: (new_pos.0 as usize, new_pos.1 as usize),
                        // previous: self.current,
                    },
                    cost,
                ));
            }
        }
        return neighbors;
    }
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true, true), "22");
    println!(
        "Part1: {}",
        part(&format!("input/{:02}_test", day), true, false)
    );
    assert_eq!(part(&format!("input/{:02}_train", day), false, true), "6,1");
    println!(
        "Part2: {}",
        part(&format!("input/{:02}_test", day), false, false)
    );
    println!("Coded: 60 Minutes");
}
