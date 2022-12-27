#[path = "common.rs"] mod common;

/// follow path of rope with head and tail
fn part1(filename: &str) -> isize {
    // read the nonsense
    let lines = common::read_lines(filename);
    let mut bleeps: Vec<String> = Vec::new();
    let mut values: Vec<isize> = Vec::new();
    let mut len: usize = 0;
    for line in lines.iter() {
        let mut parts = line.trim().split_whitespace();
        bleeps.push(parts.next().unwrap().to_string());
        match parts.next() {
            Some(value) => values.push(value.parse::<isize>().unwrap()),
            None => values.push(0),
        };
        len += 1;
    }
    let mut reg = 1isize; // register
    let mut acc = 0isize; // output accumulator

    let mut ldx = 0;
    let mut cycle = 1;
    let mut wait = 0;
    let mut next = 0isize;

    while ldx != len {
        if wait > 0 {
            // cooldown
            wait -= 1;
        } else {
            reg += next;
            // add to register and setup next
            if bleeps[ldx] == "addx" {
                // addx
                wait = 1;
                next = values[ldx];
            } else {
                // noop
                wait = 0;
                next = 0;
            }
            ldx += 1;
        }
        //println!("{:04} reg={:5} acc={} ({} {})", cycle, reg, acc, bleeps[ldx], values[ldx]);
        if (cycle == 20) || (cycle > 30 && (cycle + 20) % 40 == 0) {
            acc += reg * cycle as isize;
            //println!("{} {}", cycle, reg * cycle)
        }
        cycle += 1;
    }
    //println!("{:04} reg={:5} acc={}", cycle, reg, acc);

    return acc;
}

fn part2(filename: &str) {
    // read the nonsense
    let lines = common::read_lines(filename);
    let mut bleeps: Vec<String> = Vec::new();
    let mut values: Vec<isize> = Vec::new();
    let mut screen = vec!['.'; 6 * 40];
    let mut len: usize = 0;
    for line in lines.iter() {
        let mut parts = line.trim().split_whitespace();
        bleeps.push(parts.next().unwrap().to_string());
        match parts.next() {
            Some(value) => values.push(value.parse::<isize>().unwrap()),
            None => values.push(0),
        };
        len += 1;
    }
    let mut reg = 1isize; // register

    let mut ldx = 0;
    let mut cycle = 1;
    let mut wait = 0;
    let mut next = 0isize;

    let mut position: isize; // position on the screen

    while ldx != len {
        if wait > 0 {
            // cooldown
            wait -= 1;
        } else {
            reg += next;
            // add to register and setup next
            if bleeps[ldx] == "addx" {
                // addx
                wait = 1;
                next = values[ldx];
            } else {
                // noop
                wait = 0;
                next = 0;
            }
            ldx += 1;
        }
        position = (cycle - 1) % 240;
        if ((position - 1) % 40 == reg) || ((position) % 40 == reg) || ((position + 1) % 40 == reg) {
            screen[position as usize] = '#';
        } else {
            screen[position as usize] = '.';
        }
        cycle += 1;
    }

    // draw screen
    for rdx in 0..6 {
        for cdx in 0..40 {
            print!("{}", screen[rdx * 40 + cdx])
        }
        println!();
    }
}

pub fn solve() {
    let pdx = 10;

    // validate each solver, then apply to test
    assert_eq!(
        part1(&format!("input/{:02}_train", pdx)),
        common::read_lines_as::<isize>(&format!("input/{:02}_val1", pdx))[0]
    );
    println!("Part1: {}", part1(&format!("input/{:02}_test", pdx)));
    /*
    assert_eq!(
        part2(&format!("input/{:02}_train2", pdx), true),
        common::read_lines_as::<isize>(&format!("input/{:02}_val2", pdx))[0]
    );
    */
    part2(&format!("input/{:02}_train", pdx));
    part2(&format!("input/{:02}_test", pdx));
    /*
    println!("Part2: {}", part(&format!("input/{:02}_test", pdx), true));
    */
}
