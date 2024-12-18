use aoc;
use ndarray::Array2;

/// special push for double-wide boxes with collision detection
fn big_push(map: &mut Array2<char>, position_0: (usize, usize), diff: (i64, i64)) -> bool {
    let position_1 = (position_0.0, position_0.1 + 1);
    let next_pos_0 = (
        (position_0.0 as i64 + diff.0) as usize,
        (position_0.1 as i64 + diff.1) as usize,
    );
    let next_pos_1 = (
        (position_1.0 as i64 + diff.0) as usize,
        (position_1.1 as i64 + diff.1) as usize,
    );
    let next_char_0 = map[next_pos_0];
    let next_char_1 = map[next_pos_1];

    if next_char_0 == '#' || next_char_1 == '#' {
        // do nothing
        return false;
    } else if diff.0 == -1 || diff.0 == 1 {
        // move up or down
        if next_char_0 == '.' && next_char_1 == '.' {
            // execute move
            map[position_0] = '.';
            map[position_1] = '.';
            map[next_pos_0] = '[';
            map[next_pos_1] = ']';
            return true;
        } else if next_char_0 == '[' {
            // rock directly inline
            let ok = big_push(map, next_pos_0, diff);
            if ok {
                // able to move rock, try to move us again
                return big_push(map, position_0, diff);
            } else {
                return false;
            }
        } else if next_char_0 == ']' {
            // overlapping rock to left
            // store copy of map if things go awry
            let map_backup = map.clone();
            let ok_0 = big_push(map, (next_pos_0.0, next_pos_0.1 - 1), diff);
            let mut ok_1 = true;
            if next_char_1 == '[' {
                // also overlapping rock to right
                ok_1 = big_push(map, next_pos_1, diff);
            }
            if ok_0 && ok_1 {
                // we are actually ok and this was successful, try to move again
                return big_push(map, position_0, diff);
            } else {
                // fail, restore map
                *map = map_backup;
                return false;
            }
        } else if next_char_1 == '[' {
            // only single rock on right
            let ok = big_push(map, next_pos_1, diff);
            if ok {
                // success, retry move
                return big_push(map, position_0, diff);
            } else {
                // cannot move
                return false;
            }
        } else {
            panic!("not possible inline {}", next_char_0);
        }
    } else if diff.1 == -1 {
        // move left
        if next_char_0 == '.' {
            // execute move
            map[position_0] = '.';
            map[position_1] = '.';
            map[next_pos_0] = '[';
            map[next_pos_1] = ']';
            return true;
        } else if next_char_0 == ']' {
            let ok = big_push(map, (position_0.0, position_0.1 - 2), diff);
            if ok {
                // was able to move adjacent rock, try to move again
                return big_push(map, position_0, diff);
            } else {
                return false;
            }
        } else {
            panic!("not possible left");
        }
    } else if diff.1 == 1 {
        // move right
        if next_char_1 == '.' {
            // execute move
            map[position_0] = '.';
            map[position_1] = '.';
            map[next_pos_0] = '[';
            map[next_pos_1] = ']';
            return true;
        } else if next_char_1 == '[' {
            let ok = big_push(map, (position_0.0, position_0.1 + 2), diff);
            if ok {
                // was able to move adjacent rock, try to move again
                return big_push(map, position_0, diff);
            } else {
                return false;
            }
        } else {
            panic!("not possible right");
        }
    } else {
        panic!("i don't think this is possible")
    }
}

/// push robot and boxes
fn push(
    map: &mut Array2<char>,
    position: (usize, usize),
    diff: (i64, i64),
) -> (bool, (usize, usize)) {
    let next_pos = (
        (position.0 as i64 + diff.0) as usize,
        (position.1 as i64 + diff.1) as usize,
    );
    let this_char = map[position];
    let next_char = map[next_pos];
    if next_char == '.' {
        // make the move
        map[position] = '.';
        map[next_pos] = this_char;
        if this_char == '@' {
            return (true, next_pos);
        } else {
            return (true, (0, 0));
        }
    } else if next_char == '#' {
        // cannot move
        return (false, (0, 0));
    } else {
        // this is the location of a box; try to move it
        let ok: bool;
        match next_char {
            'O' => (ok, _) = push(map, next_pos, diff),
            '[' => ok = big_push(map, next_pos, diff),
            ']' => {
                // rock_pos is always position of '[' side
                let rock_pos: (usize, usize) = (next_pos.0, next_pos.1 - 1);
                ok = big_push(map, rock_pos, diff);
            }
            _ => unreachable!(),
        }

        if ok {
            // was able to move, try to move again
            return push(map, position, diff);
        } else {
            return (false, (0, 0));
        }
    }
}

/// Warehouse Woes
fn part(filename: &str, is_part1: bool) -> usize {
    // parse info
    let lines = aoc::read_lines(filename);
    let mut top_buffer: Vec<Vec<char>> = Vec::new();
    let mut movements: Vec<char> = Vec::new();
    for line in lines {
        if line.starts_with("#") {
            top_buffer.push(line.chars().collect());
        }
        if line.contains("^") {
            movements.extend(line.chars());
        }
    }
    // convert top_buffer to map
    let rows = top_buffer.len();
    let mut cols = top_buffer[0].len();
    // flatten into vector & convert to array2
    let mut fleeb: Vec<char> = Vec::new();
    if is_part1 {
        fleeb = top_buffer.into_iter().flatten().collect();
    } else {
        for item in top_buffer.into_iter().flatten() {
            match item {
                '#' => fleeb.extend(vec!['#', '#']),
                'O' => fleeb.extend(vec!['[', ']']),
                '.' => fleeb.extend(vec!['.', '.']),
                '@' => fleeb.extend(vec!['@', '.']),
                _ => unreachable!("character should not be present"),
            }
        }
        // twice as wide
        cols *= 2;
    }
    let mut map: Array2<char> = Array2::from_shape_vec((rows, cols), fleeb).unwrap();

    // we now have all our data
    // aoc::print_2d_chars(&map);

    let mut position: (usize, usize) = (0, 0);
    // find robot
    for ((rdx, cdx), value) in map.indexed_iter() {
        if *value == '@' {
            position = (rdx, cdx);
            break;
        }
    }
    // compute movements
    let mut ok: bool; // = false;
    let mut next_pos: (usize, usize); // = (0, 0);
    for movement in movements {
        match movement {
            '<' => (ok, next_pos) = push(&mut map, position, (0, -1)),
            '>' => (ok, next_pos) = push(&mut map, position, (0, 1)),
            '^' => (ok, next_pos) = push(&mut map, position, (-1, 0)),
            'v' => (ok, next_pos) = push(&mut map, position, (1, 0)),
            _ => unreachable!("invalid movement"),
        }
        if ok {
            position = next_pos;
        }
        // aoc::print_2d_chars(&map);
    }

    // compute box GPS
    let mut gps = 0usize;
    for ((rdx, cdx), value) in map.indexed_iter() {
        if *value == 'O' || *value == '[' {
            // encode position
            gps += 100 * rdx + cdx;
        }
    }
    return gps;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train0", day), true), 2028);
    assert_eq!(part(&format!("input/{:02}_train1", day), true), 10092);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));
    assert_eq!(part(&format!("input/{:02}_train1", day), false), 9021);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 143 Minutes");
}
