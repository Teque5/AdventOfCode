# Advent of Code

This is repository holds my solutions to the [Advent of Code](https://adventofcode.com/) puzzles.

## Using

1. `export AOC_SESSION=D34DBEEF` before running to get input dynamically from AoC server. [How to get session cookie.](https://github.com/wimglenn/advent-of-code-wim/issues/1)
2. `cd 2024; cargo run 4` runs just `Day04.rs`
3. `cargo run all` runs all days
4. `cargo run --release all` for fastest benchmark
5. `cargo test --doc` runs all tests

## Annual Thoughts

* **2024**: Rustaceans assemble! Focusing on faster solutions this year. Caching in rust feels quite clumsy compared to python's `@lru_cache` decorator.
* **2023**: Going with rust again!
* **2022**: Using this as an opportunity to learn [rust](https://www.rust-lang.org/). I did use python on some of the puzzles where I was racing against colleagues. :D

## Visualizations

### 2024 Day18: RAM Run
![2024 Day14: RAM Run](2024/img/day18.gif)

### 2024 Day14: Restroom Redoubt
![2024 Day14: Restroom Redoubt](2024/img/day14.gif)

### 2022 Day15: Beacon Exclusion Zone
![2022 Day15: Beacon Exclusion Zone](2022/img/day15.webp)

## Most Useful Crates

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
let json_line: &str = "{x=787,m=2655,a=1222,s=2876}";
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

### Requirements

* `libssl` needed for aoc input fetch
* `ffmpeg` needed for animated GIFs (2024 day14)

```bash
apt install libssl-dev ffmpeg
```
