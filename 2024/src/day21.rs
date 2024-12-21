use aoc;
use ndarray::Array2;
// use pathfinding::prelude::astar;
use pathfinding::prelude::astar_bag_collect;

use std::collections::HashMap;

/// Keypad Conundrum
///
/// keypad 1 controls keypad 2 controls keypad 3
///
/// lg_keypad 3
///
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
///
/// sm_keypad 1 & 2
///
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
fn part(filename: &str, is_part1: bool) -> usize {
    // define keypads
    let lg_keypad: HashMap<char, (usize, usize)> = vec![
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]
    .into_iter()
    .collect();
    // create keypad mazes (false = non-navigable)
    let mut lg_maze: Array2<bool> = Array2::from_elem((4, 3), true);
    lg_maze[(3, 0)] = false;
    let mut sm_maze: Array2<bool> = Array2::from_elem((2, 3), true);
    sm_maze[(0, 0)] = false;

    let direction_lut: HashMap<(isize, isize), char> = vec![
        ((-1, 0), '^'), // up
        ((1, 0), 'v'),  // down
        ((0, -1), '<'), // left
        ((0, 1), '>'),  // right
    ]
    .into_iter()
    .collect();

    let sm_keypad: HashMap<char, (usize, usize)> = vec![
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]
    .into_iter()
    .collect();

    let mut complexity = 0usize;
    // parse codes
    let code_strings = aoc::read_lines(filename);
    for code in code_strings {
        let code_numeric: usize = code[0..3].parse().unwrap();
        let code_chars: Vec<char> = code.chars().collect();
        let paths = vec![code_chars];
        let (paths, _max_len) = keypad(&paths, &lg_keypad, &lg_maze, (3, 2), &direction_lut);
        let (paths, _max_len) = keypad(&paths, &sm_keypad, &sm_maze, (0, 2), &direction_lut);
        let (paths, _max_len) = keypad(&paths, &sm_keypad, &sm_maze, (0, 2), &direction_lut);


        // println!("");
        // for possible in &paths {
        //     println!(
        //         "push_2 {} {}",
        //         possible.len(),
        //         possible.clone().into_iter().collect::<String>(),
        //     )
        // }
        complexity += _max_len * code_numeric;
    }
    return complexity;
}

/// given a prior keypad path, return next keypad path
fn keypad(
    possibilities: &Vec<Vec<char>>,
    keypad_lut: &HashMap<char, (usize, usize)>,
    keypad_maze: &Array2<bool>,
    keypad_start_position: (usize, usize),
    direction_lut: &HashMap<(isize, isize), char>,
) -> (Vec<Vec<char>>, usize) {
    let mut alpha: (usize, usize);
    let mut omega: (usize, usize);
    let mut push_1: Vec<Vec<char>> = Vec::new();
    for possible in possibilities {
        let mut fleeb: Vec<Vec<Vec<char>>> = Vec::new();
        for cdx in 0..possible.len() {
            if cdx == 0 {
                alpha = keypad_start_position; // start pointed at A!
            } else {
                alpha = *keypad_lut.get(&possible[cdx - 1]).unwrap();
            }
            omega = *keypad_lut.get(&possible[cdx]).unwrap();
            let (position_sets, _) = astar_bag_collect(
                &Position {
                    current: alpha,
                    direction: (isize::MAX, isize::MAX),
                },
                |p| p.successors(keypad_maze),
                |p| p.heuristic(omega),
                |p| p.current == omega,
            )
            .unwrap();

            let mut push_options: Vec<Vec<char>> = Vec::new();
            for positions in &position_sets {
                let mut push_set: Vec<char> = Vec::new();
                for position in positions {
                    // println!("{:?} {:?}", position.current, position.direction);
                    if position.direction.0 != isize::MAX {
                        // valid direction
                        push_set.push(*direction_lut.get(&position.direction).unwrap());
                    }
                }
                // println!("dbug {}", positions.len());
                push_set.push('A');
                push_options.push(push_set);
            }
            fleeb.push(push_options);
        }
        let fleeb_unravel = unravel_combinations(fleeb);
        push_1.extend(fleeb_unravel);
        // println!("{:?}", fleeb_unravel);
    }

    // only keep shortest paths
    let mut push_1_shortest: Vec<Vec<char>> = Vec::new();
    let mut max_len = usize::MAX;
    // find shortest path len
    for possible in &push_1 {
        if possible.len() < max_len {
            max_len = possible.len();
        }
    }
    for possible in &push_1 {
        if possible.len() == max_len {
            push_1_shortest.push(possible.clone());
        }
    }

    // for possible in &push_1_shortest {
    //     println!(
    //         "len={} path={}",
    //         possible.len(),
    //         possible.clone().into_iter().collect::<String>(),
    //     )
    // }
    return (push_1_shortest, max_len);
}

/// since our pathfinding is between chars, we need to unravel the possibilities to allow sanity in our life
fn unravel_combinations(nested_vec: Vec<Vec<Vec<char>>>) -> Vec<Vec<char>> {
    let mut combos: Vec<Vec<char>> = vec![vec![]];

    for group in nested_vec {
        let mut new_combos = Vec::new();

        for combination in &combos {
            for sub_vec in &group {
                // Extend the current combination with each sub_vec
                let mut new_combination = combination.clone();
                new_combination.extend(sub_vec);
                new_combos.push(new_combination);
            }
        }
        combos = new_combos;
    }
    combos
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    current: (usize, usize),
    direction: (isize, isize),
}

/// This Position object will be used to
/// 1. keep current position
/// 2. compute possible successors (neighbors) & cost
/// 3. compute hueristic (distance to goal)
/// 4. determine if position is inside maze
impl Position {
    fn successors(&self, maze: &Array2<bool>) -> Vec<(Position, usize)> {
        let mut neighbors = Vec::new();

        for (drow, dcol) in aoc::DIRECTIONS {
            // calculate possible position and the cost to move there
            let new_pos = (
                self.current.0 as isize + drow,
                self.current.1 as isize + dcol,
            );
            if !self.is_inside(maze.nrows(), maze.ncols(), new_pos) {
                // skip if outside maze
                continue;
            }
            let new_pos_usize = (new_pos.0 as usize, new_pos.1 as usize);
            if maze[new_pos_usize] {
                // this is a valid position
                let cost = 1usize;
                neighbors.push((
                    Position {
                        current: (new_pos.0 as usize, new_pos.1 as usize),
                        direction: (drow, dcol),
                    },
                    cost,
                ));
            }
        }
        return neighbors;
    }

    fn heuristic(&self, goal: (usize, usize)) -> usize {
        (self.current.0 as isize - goal.0 as isize).abs() as usize
            + (self.current.1 as isize - goal.1 as isize).abs() as usize
    }

    fn is_inside(&self, rows: usize, cols: usize, position: (isize, isize)) -> bool {
        position.0 >= 0
            && position.1 >= 0
            && position.0 < rows as isize
            && position.1 < cols as isize
    }
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 126384);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    // println!("Coded: 42+60*3+ Minutes");
}
