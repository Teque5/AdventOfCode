# AdventOfCode

This is repository holds my solutions to the [Advent of Code](https://adventofcode.com/) puzzles.

## 2024

Rustaceans assemble!

## 2023

Going with rust again! This year I am dynamically getting the input from the AoC servers, so we'll need to `export AOC_SESSION=D34DBEEF` before using. [How to get session cookie.](https://github.com/wimglenn/advent-of-code-wim/issues/1)

### Best Crates

#### Serde

```rust
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
// then later
let some_part: Part = serde_json::from_str(&json_line).unwrap();
// and it adds the debug print so you can
println!("{:?}", some_part)
```

#### ProgressBar

```rust
let max = 777u64;
use indicatif::{ProgressBar, ProgressStyle};
let progress = ProgressBar::new(max);
progress.set_style(
    ProgressStyle::with_template(
        "{bar:40.cyan/blue} {pos:>9}/{len:9} [{eta} left] {msg}",
    ).unwrap(),
);
progress.set_message("finding repeating sequence...");
for idx in 0..max {
    // do something
    progress.set_position(idx as u64);
}
progress.finish_with_message("Done");
```

#### Rayon

```rust
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
acc = AtomicUsize::new(0);
lines.par_iter().for_each(|line| {
    // do something
    acc.fetch_add(1, Ordering::Relaxed);
})
let final_acc = acc.load(Ordering::Relaxed);
```

### Dependencies

```bash
# 2023+ Requirement
apt install libssl-dev
```

## 2022

Using this as an opportunity to learn [rust](https://www.rust-lang.org/). I did use python on some of the puzzles where I was racing against colleagues. :D

![day15](2022/img/day15.webp)
