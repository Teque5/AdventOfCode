#[path = "common.rs"] mod common;

use ndarray::Array2;
use ndarray::s;

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
        //println!("{} {}", dulrs[0], dists[0]);
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
            _ => println!("err")
        }
        if rpos < rmin {rmin = rpos}
        if rpos > rmax {rmax = rpos}
        if cpos < cmin {cmin = cpos}
        if cpos > cmax {cmax = cpos}
    }
    // start position is ???
   // println!("drow {} {}", rmin, rmax);
 //   println!("dcol {} {}", cmin, cmax);
//    println!("dpos {} {}", cpos, cpos);
    let startpos = (-rmin as usize, -cmin as usize); // is this correct?
    let mut headpos = startpos;
    let mut tailpos = startpos;

    // track the position of the head & tail, count tail touches
    // 0 is unvisited, 1 is tail visited
//    println!("ray size {} {}", (rmax-rmin+1), (cmax-cmin+1));
    println!("star({}, {})", startpos.0, startpos.1);
    let mut visit: Array2<usize> = Array2::zeros(((rmax-rmin+1) as usize, (cmax-cmin+1) as usize));
    for ldx in 0..len {
        let dist = dists[ldx as usize] as usize;
        match dulrs[ldx as usize] {
            'D' => {
//                println!("zz({}, {}) {}", headpos.0, headpos.1, dist);
                headpos.0 += dist;
                if headpos.0 > tailpos.0 + 1 {
                    tailpos.0 = headpos.0 - 1;
                    tailpos.1 = headpos.1;

                }
//                visit.slice_mut(s![headpos.0..headpos.0+dist, headpos.1]).fill(1);

                
            }
            'U' => {
                headpos.0 -= dist;
            }
            'L' => {
                headpos.1 -= dist;
            }
            'R' => {
                headpos.1 += dist;
                
            }
            _ => println!("err")
        }
        println!("head({}, {}) {} {}", headpos.0, headpos.1, dulrs[ldx], dists[ldx]);
    }
    for vrow in visit.outer_iter() {
        for vitem in vrow.iter() {
            print!("{} ", vitem)
        }
        println!("")
    }

    return 0
}

pub fn solve() {
    let pdx = 9;

    // validate each solver, then apply to test
    assert_eq!(
        part(&format!("input/{:02}_train", pdx), false),
        common::read_lines_as::<i32>(&format!("input/{:02}_val1", pdx))[0]
    );
//    println!("Part1: {}", part(&format!("input/{:02}_test", pdx), false));

    // part 2
//    assert_eq!(
//        part(&format!("input/{:02}_train", pdx), true),
//        common::read_lines_as::<i32>(&format!("input/{:02}_val2", pdx))[0]
//    );
//    println!("Part2: {}", part(&format!("input/{:02}_test", pdx), true));
}
