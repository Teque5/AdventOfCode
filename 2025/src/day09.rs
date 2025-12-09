use aoc;

/// Movie Theater
fn part(filename: &str, is_part1: bool) -> usize {
    let lines = aoc::read_lines(filename);
    let mut tiles = Vec::<(usize, usize)>::new();
    for line in lines.iter() {
        let bla = aoc::parse_delimited::<usize>(line, ',');
        tiles.push((bla[0], bla[1]));
    }
    let mut largest_area = 0usize;
    // n choose 2
    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let dx = (tiles[i].0 as isize - tiles[j].0 as isize).abs() + 1;
            let dy = (tiles[i].1 as isize - tiles[j].1 as isize).abs() + 1;
            let area = (dx * dy) as usize;
            if area > largest_area {
                largest_area = area;
            }
            // println!("Comparing {:?} and {:?}: dx={}, dy={}", tiles[i], tiles[j], dx, dy);
        }
    }
    // for tile in tiles.iter() {
    //     println!("{:?}", tile);
    // }

    return largest_area;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train", day), true), 50);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    // assert_eq!(part(&format!("input/{:02}_train", day), false), 123);
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: XX minutes.");
}
