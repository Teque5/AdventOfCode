#[path = "common.rs"] mod common;

use ndarray::array;
use ndarray::s;
use ndarray::Array2;
use ndarray::ArrayViewMut2;

/// follow path of rope with head and tail
fn part(filename: &str, part2: bool) -> i32 {
    // read the nonsense
    let lines = common::read_lines(filename);
    let mut dulrs: Vec<char> = Vec::new();
    let mut dists: Vec<isize> = Vec::new();
    let mut len: usize = 0;
    for line in lines.iter() {
        let mut parts = line.trim().split_whitespace();
        // this seems like a lot of work to just parse "R 4"
        dulrs.push(parts.next().unwrap().chars().nth(0).unwrap());
        dists.push(parts.next().unwrap().parse::<isize>().unwrap());
        len += 1;
    }
    // determine how big of a 2d matrix we need
    let (mut rmin, mut rmax) = (0isize, 0isize);
    let (mut cmin, mut cmax) = (0isize, 0isize);
    let (mut rpos, mut cpos) = (0isize, 0isize);
    for ldx in 0..len {
        let dist = dists[ldx as usize];
        match dulrs[ldx as usize] {
            'D' => rpos += dist,
            'U' => rpos -= dist,
            'L' => cpos -= dist,
            'R' => cpos += dist,
            _ => println!("err"),
        }
        if rpos < rmin {rmin = rpos}
        if rpos > rmax {rmax = rpos}
        if cpos < cmin {cmin = cpos}
        if cpos > cmax {cmax = cpos}
    }
    let mut visit: Array2<usize> = Array2::zeros(((rmax - rmin + 1) as usize, (cmax - cmin + 1) as usize));
    if !part2 {
        // part 1
        let mut headpos = (-rmin as usize, -cmin as usize);
        let mut tailpos = headpos;

        // track the position of the head & tail, count tail touches
        for ldx in 0..len {
            let dist = dists[ldx as usize] as usize;
            for _ in 0..dist {
                step_part1(&mut visit, &mut headpos, &mut tailpos, dulrs[ldx]);
            }
        }
    } else {
        // part 2 is rope with 9 segment pairs instead of just 1
        let mut rope: Array2<usize> = Array2::zeros((10, 2));
        for mut knot in rope.outer_iter_mut() {
            knot.assign(&array![-rmin as usize, -cmin as usize]);
        }
        // iterate over movements
        for ldx in 0..len {
            let dist = dists[ldx as usize] as usize;
            // iterate over length traveled
            for _ in 0..dist {
                // move head
                step_head(dulrs[ldx], &mut rope);
                // update rest of rope segments
                for kdx in 0..9 {
                    let mut headtail = rope.slice_mut(s![kdx..kdx + 2, ..]);
                    step_part2(&mut headtail);
                    if kdx == 8 {
                        // track rope final bit only
                        visit[[headtail[[1, 0]], headtail[[1, 1]]]] = 1
                    }
                    //println!("   k{} ({}, {})", kdx, rope[[kdx,0]], rope[[kdx,1]])
                }
            }
        }
    }
    //if visit.len() < 1000 {print_state(&visit)}
    return visit.sum() as i32;
}

fn step_head(direction: char, head: &mut Array2<usize>) {
    match direction {
        'D' => head[[0, 0]] += 1,
        'U' => head[[0, 0]] -= 1,
        'L' => head[[0, 1]] -= 1,
        'R' => head[[0, 1]] += 1,
        _ => println!("err {}", direction),
    }
}

/// return separate row, col distance
fn rc_dist(headtail: &mut ArrayViewMut2<usize>) -> (isize, isize) {
    (
        ((headtail[[0, 0]] as isize) - (headtail[[1, 0]] as isize)),
        ((headtail[[0, 1]] as isize) - (headtail[[1, 1]] as isize)),
    )
}

/*
fn print_state(visit: &Array2<usize>) {
    for vrow in visit.outer_iter() {
        for vitem in vrow.iter() {
            print!("{}", vitem)
        }
        println!("")
    }
}
*/

/// return new knot positions
/// headtail is [[head, tail]] mutable array
fn step_part2(headtail: &mut ArrayViewMut2<usize>) {
    let (rdist, cdist) = rc_dist(headtail);
    // these diagonal moves caused me much mental pain but can likely be described simpler
    if (cdist == 2) && (rdist == 2) {
        // downleft
        headtail[[1, 0]] = headtail[[0, 0]] - 1;
        headtail[[1, 1]] = headtail[[0, 1]] - 1;
    } else if (cdist == 2) && (rdist == -2) {
        // upleft
        headtail[[1, 0]] = headtail[[0, 0]] + 1;
        headtail[[1, 1]] = headtail[[0, 1]] - 1;
    } else if (cdist == -2) && (rdist == 2) {
        // downright
        headtail[[1, 0]] = headtail[[0, 0]] - 1;
        headtail[[1, 1]] = headtail[[0, 1]] + 1;
    } else if (cdist == -2) && (rdist == -2) {
        // upright
        headtail[[1, 0]] = headtail[[0, 0]] + 1;
        headtail[[1, 1]] = headtail[[0, 1]] + 1;
    } else if rdist == 2 {
        // down
        headtail[[1, 0]] = headtail[[0, 0]] - 1;
        headtail[[1, 1]] = headtail[[0, 1]];
    } else if rdist == -2 {
        // up
        headtail[[1, 0]] = headtail[[0, 0]] + 1;
        headtail[[1, 1]] = headtail[[0, 1]];
    } else if cdist == 2 {
        // left
        headtail[[1, 0]] = headtail[[0, 0]];
        headtail[[1, 1]] = headtail[[0, 1]] - 1;
    } else if cdist == -2 {
        // right
        headtail[[1, 0]] = headtail[[0, 0]];
        headtail[[1, 1]] = headtail[[0, 1]] + 1;
    }
}

/// tail vaguely tracks the head as it wanders around
fn step_part1(
    visit: &mut Array2<usize>,
    headpos: &mut (usize, usize),
    tailpos: &mut (usize, usize),
    direction: char,
) {
    match direction {
        'D' => {
            headpos.0 += 1;
            if l2dist(&headpos, &tailpos) >= 2. {
                tailpos.0 = headpos.0 - 1;
                tailpos.1 = headpos.1;
            }
        }
        'U' => {
            headpos.0 -= 1;
            if l2dist(&headpos, &tailpos) >= 2. {
                tailpos.0 = headpos.0 + 1;
                tailpos.1 = headpos.1;
            }
        }
        'L' => {
            headpos.1 -= 1;
            if l2dist(&headpos, &tailpos) >= 2. {
                tailpos.0 = headpos.0;
                tailpos.1 = headpos.1 + 1;
            }
        }
        'R' => {
            headpos.1 += 1;
            if l2dist(&headpos, &tailpos) >= 2. {
                tailpos.0 = headpos.0;
                tailpos.1 = headpos.1 - 1;
            }
        }
        _ => println!("err"),
    }
    visit[*tailpos] = 1;
}

/// L2 Norm Distance
fn l2dist(pos_a: &(usize, usize), pos_b: &(usize, usize)) -> f32 {
    (((pos_a.0 as f32) - (pos_b.0 as f32)).powi(2) + ((pos_a.1 as f32) - (pos_b.1 as f32)).powi(2))
        .sqrt()
}

pub fn solve() {
    let pdx = 9;

    // validate each solver, then apply to test
    assert_eq!(
        part(&format!("input/{:02}_train1", pdx), false),
        common::read_lines_as::<i32>(&format!("input/{:02}_val1", pdx))[0]
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", pdx), false));

    assert_eq!(
        part(&format!("input/{:02}_train2", pdx), true),
        common::read_lines_as::<i32>(&format!("input/{:02}_val2", pdx))[0]
    );
    println!("Part2: {}", part(&format!("input/{:02}_test", pdx), true));
}
