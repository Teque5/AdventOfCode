use aoc;
use ndarray::Array2;
use pathfinding::grid::Grid;

/// Movie Theater
fn part(filename: &str, is_part1: bool) -> usize {
    let lines = aoc::read_lines(filename);
    let mut tiles = Vec::<(usize, usize)>::new();
    let (mut rows, mut cols) = (0usize, 0usize);
    for line in lines.iter() {
        let bla = aoc::parse_delimited::<usize>(line, ',');
        tiles.push((bla[0], bla[1]));
        if bla[0] + 1 > rows {
            rows = bla[0] + 1;
        }
        if bla[1] + 1 > cols {
            cols = bla[1] + 1;
        }
    }
    if is_part1 {
        // println!("Grid size: {} x {}", rows, cols);
        let mut largest_area = 0usize;
        // n choose 2
        for idx in 0..tiles.len() {
            for jdx in idx + 1..tiles.len() {
                let dx = (tiles[idx].0 as isize - tiles[jdx].0 as isize).abs() + 1;
                let dy = (tiles[idx].1 as isize - tiles[jdx].1 as isize).abs() + 1;
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
        return largest_area
    }
    // part 2. construct giant grid and fill tiles
    // TODO: I believe we need to connect our tiles with vertices along edges
    // copy first tile to end of tiles because it wraps around now
    tiles.push(tiles[0]);
    // let mut bigboy = Array2::<bool>::from_elem((rows, cols), false);
    let mut grid = Grid::new(rows, cols);
    for tile in tiles.iter() {
        grid.add_vertex((tile.0, tile.1));
    }
    for edge in grid.edges() {

        println!("edge {:?}", edge);
    }

    let mut testing = Array2::<usize>::from_elem((rows, cols), 0);
    for rdx in 0..rows {
        for cdx in 0..cols {
            if grid.is_inside((rdx, cdx)) {
                testing[[rdx, cdx]] = 1;
            }
        }
    }
    aoc::print_2d(&testing);


    println!("{:?}", grid);
    // aoc::print_2d(&bigboy2);
    // // paint fill the enclosed area

    // // convert bigboy to usize
    // let bigboy2 = bigboy.mapv(|b| if b { 1usize } else { 0usize });
    // println!("");
    // aoc::print_2d(&bigboy2);
    return 0;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    // assert_eq!(part(&format!("input/{:02}_train", day), true), 50);
    // println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train", day), false), 24);
    // println!("Part2: {}", part(&format!("input/{:02}_test", day), false));

    println!("Coded: XX minutes.");
}
