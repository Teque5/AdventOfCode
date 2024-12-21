use aoc;
use ndarray::{s, Array2};

#[rustfmt::skip]
fn is_in_map(rows: usize, cols: usize, position: (isize, isize)) -> bool {
    position.0 >= 0
    && position.1 >= 0
    && position.0 < rows as isize
    && position.1 < cols as isize
}

/// return array of valid directions from some position
fn matching_directions(
    garden: &Array2<char>,
    position: &(usize, usize),
    current: &char,
    rows: usize,
    cols: usize,
) -> Vec<(usize, usize)> {
    let mut possible: Vec<(usize, usize)> = Vec::new();
    for direction in aoc::DIRECTIONS {
        let new_position = (
            position.0 as isize + direction.0,
            position.1 as isize + direction.1,
        );
        if is_in_map(rows, cols, new_position) {
            let new_usize_position = (new_position.0 as usize, new_position.1 as usize);
            if *current == garden[new_usize_position] {
                possible.push(new_usize_position);
            }
        }
    }
    return possible;
}

/// Severely dissapointing that rust doesn't have a good way to do simple Conv2d()
fn convolve_2d(input: &Array2<f32>, kernel: &Array2<f32>) -> Array2<f32> {
    let (input_rows, input_cols) = input.dim();
    let (kernel_rows, kernel_cols) = kernel.dim();

    // create padding
    let pad_rows = kernel_rows - 1;
    let pad_cols = kernel_cols - 1;
    let mut padded_input =
        Array2::<f32>::zeros((input_rows + 2 * pad_rows, input_cols + 2 * pad_cols));

    // copy input to the center of the padding
    padded_input
        .slice_mut(s![
            pad_rows..pad_rows + input_rows,
            pad_cols..pad_cols + input_cols
        ])
        .assign(input);

    // init output
    let output_rows = input_rows + kernel_rows - 1;
    let output_cols = input_cols + kernel_cols - 1;
    let mut output = Array2::<f32>::zeros((output_rows, output_cols));

    // convolve
    for row in 0..output_rows {
        for col in 0..output_cols {
            // region of the padded input that matches the kernel size
            let region = padded_input.slice(s![row..row + kernel_rows, col..col + kernel_cols,]);

            // element-wise multiplication and summation
            let sum = (region.to_owned() * kernel).sum();
            output[[row, col]] = sum;
        }
    }

    output
}

/// Garden Groups
/// For part2 the trick is that num_corners == num_sides
/// Computing the number of corners for part 2 was hell.
/// A hint to use convolution saved my ass.
/// I re-wrote part2 at least three full times.
/// Also added some extra training grids to figure out edge cases
fn part(filename: &str, is_part1: bool) -> usize {
    let mut price = 0usize;
    // parse info
    let (garden, rows, cols) = aoc::read_2d_chars(filename);
    // keep track of where we have already processed
    let mut traversed: Array2<bool> = Array2::from_elem((rows, cols), false);
    for rdx in 0..rows {
        for cdx in 0..cols {
            if traversed[(rdx, cdx)] {
                continue;
            }
            // process garden from new position
            traversed[(rdx, cdx)] = true;
            let region_name = garden[(rdx, cdx)];
            // keep track of this particular region for part 2 where we need to count vertices
            let mut this_region: Array2<bool> = Array2::from_elem((rows, cols), false);
            this_region[(rdx, cdx)] = true;
            let mut area = 1usize;
            let mut perimeter = 4usize;
            let mut possible = matching_directions(&garden, &(rdx, cdx), &region_name, rows, cols);
            while let Some(position) = possible.pop() {
                if traversed[position] {
                    continue;
                }
                // grow garden to this position
                traversed[position] = true;
                this_region[position] = true;
                // how many other directions from here match current garden?
                let new_possibilities =
                    matching_directions(&garden, &position, &region_name, rows, cols);
                let mut num_traversed_adjacent = 0usize;
                for poss in &new_possibilities {
                    if traversed[*poss] {
                        num_traversed_adjacent += 1;
                    }
                }
                match num_traversed_adjacent {
                    0 => {} // no perimeter change
                    1 => perimeter += 2,
                    2 => {} // no perimeter change
                    3 => perimeter -= 2,
                    4 => perimeter -= 4,
                    _ => panic!("not possible"),
                }
                area += 1;
                possible.extend(new_possibilities);
            }
            if is_part1 {
                price += area * perimeter;
            } else {
                // determine number of corners, which equals number of edges
                // use 2d convolution
                let region: Array2<f32> = this_region.map(|&b| if b { 1.0 } else { 0.0 });
                let kernel: Array2<f32> =
                    Array2::from_shape_vec((2, 2), vec![-1.0, 1.0, 1.0, -1.0]).unwrap();
                let result = convolve_2d(&region, &kernel);
                // println!("conv result");
                // for row in result.rows() {
                //     println!("{:?}", row);
                // }
                let corners = result.abs().sum() as usize;
                price += area * corners as usize;
            }
        }
    }
    return price;
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(part(&format!("input/{:02}_train0", day), true), 140);
    assert_eq!(part(&format!("input/{:02}_train1", day), true), 772);
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));

    assert_eq!(part(&format!("input/{:02}_train0", day), false), 80);
    assert_eq!(part(&format!("input/{:02}_train1", day), false), 436);
    assert_eq!(part(&format!("input/{:02}_train2", day), false), 236);
    assert_eq!(part(&format!("input/{:02}_train3", day), false), 1206);
    assert_eq!(part(&format!("input/{:02}_train4", day), false), 368);
    assert_eq!(part(&format!("input/{:02}_train5", day), false), 414);
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 395 Minutes");
}
