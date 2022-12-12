#[path = "common.rs"] mod common;


fn part1(filename: &str) -> &str {
     let mut acc = 0i32;
    // read each pair
    let lines = common::read_lines(filename);
    for line in lines.iter() {
        let pack = line.as_bytes();
        // reshape into chunks
        // not sure why there is an underscore in this vec
        let chunks: Vec<_> = pack.chunks(pack.len() / 2).collect();

        for c0 in chunks[0] {
            if chunks[1].contains(c0) {
                let ccc = *c0 as i32;
                // resolve value of shared item
                if ccc > 94 {
                    // lowercase
                    acc += ccc - 96;
 //                   println!("l {}", ccc-96);
                    break;
                } else {
                    // uppercase
                    acc += ccc - 38;
   //                 println!("u {}", ccc-38);
                    break;
                }
            }

        }
    }
    return "ASDF";
}


pub fn solve() {
    // validate each solver, then apply to test 
    assert_eq!(part1("input/05_train"), common::read_lines("input/05_val1")[0]);
    println!();
    println!("Part1: {}", part1("input/05_test"));

//    assert_eq!(part2("input/03_train"), common::read_lines_as::<i32>("input/03_val2")[0]);
//    println!("Part2: {}", part2("input/03_test"));

}
