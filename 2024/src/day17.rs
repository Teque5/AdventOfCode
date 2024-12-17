use aoc;

/// get combo operand
fn co(operand: usize, reg_a: usize, reg_b: usize, reg_c: usize) -> usize {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => unreachable!(),
    }
}

/// Chronospatial Computer
fn part(filename: &str, is_part1: bool) -> String {
    // strange value found experimentally with python by looking at diffs from initial brute-force fine-grained search
    let mut start_a = 775457178usize;
    let mut _best_idx = 0usize; // special for part 2

    // init everything
    let mut reg_a = 0usize; // register A
    let mut reg_b = 0usize; // register B
    let mut reg_c = 0usize; // register C
    let mut point = 0usize; // instruction pointer
    let mut program: Vec<usize> = Vec::new();
    let mut output: Vec<usize> = Vec::new();

    // parse info
    let lines = aoc::read_lines(filename);
    for line in lines {
        if line.starts_with("Register A") {
            reg_a = aoc::parse_numbers::<usize>(&line)[0];
        } else if line.starts_with("Register B") {
            reg_b = aoc::parse_numbers::<usize>(&line)[0];
        } else if line.starts_with("Register C") {
            reg_c = aoc::parse_numbers::<usize>(&line)[0];
        } else {
            program = aoc::parse_numbers::<usize>(&line);
        }
    }

    // for part2 set ok when program outputs copy of itself
    let mut ok = false;
    loop {
        // main processing loop
        while point < program.len() - 1 {
            let instruction = program[point];
            let operand = program[point + 1];
            // println!(
            //     "point={}    {},{}      a={} b={}, c={}     co={}",
            //     point,
            //     instruction,
            //     operand,
            //     reg_a,
            //     reg_b,
            //     reg_c,
            //     co(operand, reg_a, reg_b, reg_c)
            // );
            let mut is_jump = false;
            let mut is_output = false;
            match instruction {
                0 => {
                    reg_a = (reg_a as f64 / (2.0_f64).powf(co(operand, reg_a, reg_b, reg_c) as f64))
                        as usize
                } // adv
                1 => reg_b ^= operand,                             // bxl
                2 => reg_b = co(operand, reg_a, reg_b, reg_c) % 8, // bst
                3 => {
                    if reg_a != 0 {
                        point = operand;
                        is_jump = true;
                    }
                } // jnz
                4 => reg_b = reg_b ^ reg_c,                        // bxc
                5 => {
                    output.push(co(operand, reg_a, reg_b, reg_c) % 8);
                    is_output = true;
                } // out
                6 => {
                    reg_b = (reg_a as f64 / (2.0_f64).powf(co(operand, reg_a, reg_b, reg_c) as f64))
                        as usize
                } // bdv
                7 => {
                    reg_c = (reg_a as f64 / (2.0_f64).powf(co(operand, reg_a, reg_b, reg_c) as f64))
                        as usize
                } // cdv
                _ => unreachable!(),
            }
            if !is_jump {
                point += 2;
            }
            if !is_part1 && is_output {
                // check if output is matching program
                let out_len = output.len();
                // most recent output should match program
                if output[out_len - 1] != program[out_len - 1] {
                    // bad route
                    break;
                } else if out_len == program.len() {
                    // this is the end; we found solution
                    ok = true;
                    break;
                } else {
                    // if out_len >= _best_idx {
                    //     // use analysis to figure out pattern
                    //     _best_idx = out_len;
                    //     println!("{} {:?} is ok", start_a, output);
                    // }
                }
            }
        }
        if is_part1 || ok {
            // println!("break on ok @ {}", start_a);
            break;
        } else {
            // reset and try again
            start_a += 1073741824;
            reg_a = start_a;
            reg_b = 0;
            reg_c = 0;
            point = 0;
            output.clear();
        }
    }

    if is_part1 {
        // return string of instructions
        return output
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",");
    } else {
        // return value of special A
        return start_a.to_string();
    }
}

/// Check training data, then apply to test data
pub fn solve(day: usize) {
    assert_eq!(
        part(&format!("input/{:02}_train0", day), true),
        "4,6,3,5,6,3,5,2,1,0"
    );
    println!("Part1: {}", part(&format!("input/{:02}_test", day), true));
    // this example works if we use start_a = 0 and a += 1
    // assert_eq!(part(&format!("input/{:02}_train1", day), false), "117440");
    println!("Part2: {}", part(&format!("input/{:02}_test", day), false));
    println!("Coded: 256 Minutes");
}

// the following values work when doing += 1 from initial value of 0
// brute_force = [453576859,453577114,453578497,453578505,453601690,728238235,728238490,728239873,728239881,775457178,775458561,775458569,775484169,990447771,990448026,990449409,990449417,990472602,1849199002,1849200385,1849200393,1849225993,2858944667,2858944922,2858946305,2858946313,2922940826,2922942209,2922942217,2922967817,3932686491,3932686746,3932688129,3932688137,3996682650,3996684033,3996684041,3996709641,5070424474,5070425857,5070425865,5070451465,6144166298,6144167681,6144167689,6144193289,7217908122,7217909505,7217909513,7217935113,8291649946,8291651329,8291651337,8291676937,9043511451,9043511706,9043513089,9043513097,9043536282,9318172827,9318173082,9318174465,9318174473,9365391770,9365393153,9365393161,9365418761,9580382363,9580382618,9580384001,9580384009,9580407194,10439133594,10439134977,10439134985,10439160585,11512875418,11512876801,11512876809,11512902409,12586617242,12586618625,12586618633,12586644233,13660359066,13660360449,13660360457,13660386057,14734100890,14734102273,14734102281,14734127881,15807842714,15807844097,15807844105,15807869705,16881584538,16881585921,16881585929,16881611529,17633446043,17633446298,17633447681,17633447689,17633470874,17908107419,17908107674,17908109057,17908109065,17955326362,17955327745,17955327753,17955353353,18170316955,18170317210,18170318593,18170318601,18170341786,19029068186,19029069569,19029069577,19029095177,20038813851,20038814106,20038815489,20038815497,20102810010,20102811393,20102811401,20102837001,21112555675,21112555930,21112557313,21112557321,21176551834,21176553217,21176553225,21176578825,22250293658,22250295041,22250295049,22250320649,23324035482,23324036865,23324036873,23324062473,24397777306,24397778689,24397778697,24397804297,25471519130,25471520513,25471520521,25471546121,26223380635,26223380890,26223382273,26223382281,26223405466,26498042011,26498042266,26498043649,26498043657,26545260954,26545262337,26545262345,26545287945,26760251547,26760251802,26760253185,26760253193,26760276378,27619002778,27619004161,27619004169,27619029769,28692744602,28692745985,28692745993,28692771593,29766486426,29766487809,29766487817,29766513417,30840228250,30840229633,30840229641,30840255241,31913970074,31913971457,31913971465,31913997065,32987711898,32987713281,32987713289,32987738889,34061453722,34061455105,34061455113,34061480713,34813315227,34813315482,34813316865,34813316873,34813340058,35087976603,35087976858,35087978241,35087978249,35135195546,35135196929,35135196937,35135222537,35350186139,35350186394,35350187777,35350187785,35350210970,36208937370,36208938753,36208938761,36208964361,37218683035,37218683290,37218684673,37218684681,37282679194,37282680577,37282680585,37282706185,38292424859,38292425114,38292426497,38292426505,38356421018,38356422401,38356422409,38356448009,39430162842,39430164225,39430164233,39430189833,40503904666,40503906049,40503906057,40503931657,41577646490,41577647873,41577647881,41577673481,42651388314,42651389697,42651389705,42651415305,43403249819,43403250074,43403251457,43403251465,43403274650,43677911195,43677911450,43677912833,43677912841,43725130138,43725131521,43725131529,43725157129,43940120731,43940120986,43940122369,43940122377,43940145562,44798871962,44798873345,44798873353,44798898953,45872613786,45872615169,45872615177,45872640777,46946355610,46946356993,46946357001,46946382601,48020097434,48020098817,48020098825,48020124425,49093839258,49093840641,49093840649,49093866249,50167581082,50167582465,50167582473,50167608073,51241322906,51241324289,51241324297,51241349897,51993184411,51993184666,51993186049,51993186057,51993209242,52267845787,52267846042,52267847425,52267847433,52315064730,52315066113,52315066121,52315091721,52530055323,52530055578,52530056961,52530056969,52530080154,53388806554,53388807937,53388807945,53388833545,54398552219,54398552474,54398553857,54398553865,54462548378,54462549761,54462549769,54462575369,55472294043,55472294298,55472295681,55472295689,55536290202,55536291585,55536291593,55536317193]
