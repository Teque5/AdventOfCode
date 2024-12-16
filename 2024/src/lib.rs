/// Library functions for Advent of Code solutions
/// Copyright 2021 Alex Utter, 2022-2024 Teque5
use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use ndarray::{Array2, Axis};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;
use std::process::Command;
use std::str::FromStr;
use tempfile::{tempdir, TempDir};

/// commonly used directions (row, col) for mazes and whatnot
pub const DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), // up
    (1, 0),  // down
    (0, -1), // left
    (0, 1),  // right
];

/// style string for indicatif::ProgressBar
pub const STYLE: &str = "{bar:40.cyan/blue} {pos:>9}/{len:9} [{eta} left] {msg}";

/// read a file into a vector of strings
pub fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines.filter_map(Result::ok).collect()
}

/// read a file with one number per line
pub fn read_lines_as<T: FromStr>(filename: &str) -> Vec<T> {
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines
        .filter_map(Result::ok)
        .filter_map(|line| line.trim().parse::<T>().ok())
        .collect()
}

/// read a file with rectangular text block as a 2d array of chars
/// 123
/// 456
/// 789
pub fn read_2d_chars(filename: &str) -> (Array2<char>, usize, usize) {
    let mut file = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    // read file to bytes
    file.read_to_end(&mut buffer).unwrap();
    // 10 is the line feed ascii decimal
    let cols = match buffer.iter().position(|&c| c == 10) {
        Some(pos) => pos,
        None => panic!("no line feed found"),
    };
    // printable characters are above 32
    buffer.retain(|&x| x >= 32);
    let charbuffer = buffer.iter().map(|b| *b as char).collect::<Vec<_>>();
    let rows = charbuffer.len() / cols;
    // println!("dbug {} {} {}", rows, cols, charbuffer.len());
    let ray = Array2::from_shape_vec((rows, cols), charbuffer).unwrap();
    return (ray, rows, cols);
}

/// print 2d array of chars
#[allow(dead_code)]
pub fn print_2d_chars(ray: &Array2<char>) {
    for row in ray.axis_iter(Axis(0)) {
        let some_string: String = row.into_iter().collect();
        println!("{:}", some_string);
    }
}

/// parse character-delimited string as type
///
/// ## Examples
/// ```
/// use aoc::parse_delimited;
/// let result = parse_delimited::<isize>("1,-30,4,5", ',');
/// assert_eq!(result, [1, -30, 4, 5]);
/// let result = parse_delimited::<f64>("0|1|2|3", '|');
/// assert_eq!(result, [0.0, 1.0, 2.0, 3.0]);
/// let result = parse_delimited::<u8>("0,255,256,", ',');
/// assert_eq!(result, [0, 255]);
/// ```
pub fn parse_delimited<T: FromStr>(line: &str, delim: char) -> Vec<T> {
    line.split(delim)
        .filter_map(|x| x.parse::<T>().ok())
        .collect()
}

/// parse numbers (fuzzy)
/// ignore text, and return +/- single or multi-digit numbers
///
/// ## Examples
/// ```
/// use aoc::parse_numbers;
/// let result = parse_numbers::<i8>("1 ⍼:: whatever-30 4 5");
/// assert_eq!(result, [1, -30, 4, 5]);
/// let result = parse_numbers::<u64>("Prize: X=8400, Y=5400");
/// assert_eq!(result, [8400, 5400]);
/// let result = parse_numbers::<isize>("<123>|<-1> 2&3");
/// assert_eq!(result, [123, -1, 2, 3]);
/// let result = parse_numbers::<f32>("123.456x789,0.5");
/// assert_eq!(result, [123.456, 789.0, 0.5]);
/// ```
pub fn parse_numbers<T: FromStr>(line: &str) -> Vec<T> {
    line.split(|c: char| !c.is_digit(10) && c != '-' && c != '.')
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<T>().ok())
        .collect()
}

#[allow(dead_code)]
pub struct Image<'a> {
    img: RgbImage,
    font: FontRef<'a>,
    scale: PxScale,
    dir: TempDir,
    rows: usize,
    cols: usize,
    fdx: u16,      // frame index
    alpha: f32,    // fade alpha
    framerate: u8, // gif framerate
}

/// Very often in AoC we have an Array2<_> where we want to visualize the changes
/// ## Example (not doctest)
/// let img = aoc::Image::new(10, 20);
/// for _ in 0..10 {
///     img.draw_chars(&some_2d_array);
///     img.render_frame()
///     img.fade()
/// }
/// img.render_gif("out.gif");
#[allow(dead_code)]
impl Image<'_> {
    /// create new image surface
    pub fn new(rows: usize, cols: usize) -> Self {
        // font needs to be monospace
        let font = FontRef::try_from_slice(include_bytes!("../../unscii-8.otf")).unwrap();
        let scale = PxScale {
            x: 8.0, // fixed for unscii-8
            y: 8.0, // fixed for unscii-8
        };
        let img = RgbImage::new(
            (scale.x as usize * cols) as u32,
            (scale.y as usize * rows) as u32,
        );
        // TODO: handle tempdir()? instead of unwrapping
        let dir = tempdir().unwrap();
        Image {
            img: img,
            font: font,
            scale: scale,
            dir: dir,
            rows: rows,
            cols: cols,
            fdx: 0,
            alpha: 0.05,
            framerate: 15,
        }
        // TODO: check that ffmpeg is installed
    }
    /// clear image and redefine character size
    /// 8 (default) is pixel-perfect for text
    /// 4 is barely resolvable
    /// 1 is for tiny pixel plotting using draw_bool
    pub fn set_fontsize(&mut self, size_px: f32) {
        self.scale.x = size_px;
        self.scale.y = size_px;
        self.img = RgbImage::new(
            (self.scale.x as usize * self.cols) as u32,
            (self.scale.y as usize * self.rows) as u32,
        );
    }

    // drawing subroutines

    /// fade every pixel to emphasize any new frames
    pub fn fade(&mut self) {
        // fade old image into red channel
        for pixel in self.img.pixels_mut() {
            pixel[0] = (pixel[0] as f32 * (1.0 - self.alpha)) as u8;
            pixel[1] = 0;
            pixel[2] = 0;
        }
    }

    /// draw exactly 1 or more chars at (row, col)
    pub fn draw_text(&mut self, row: usize, col: usize, txt: &str) {
        let y_offset = (row as f32 * self.scale.y) as i32;
        let x_offset = (col as f32 * self.scale.y) as i32;
        // draw black background (not really needed)
        // let blank: String = "█".repeat(txt.len());
        // draw_text_mut(
        //     &mut self.img,
        //     Rgb([0, 0, 0]),
        //     x_offset,
        //     y_offset,
        //     self.scale,
        //     &self.font,
        //     &blank,
        // );
        draw_text_mut(
            &mut self.img,
            Rgb([255, 255, 255]),
            x_offset,
            y_offset,
            self.scale,
            &self.font,
            txt,
        );
    }

    /// draw a full or empty value at (row, col)
    pub fn draw_bool(&mut self, row: usize, col: usize, value: bool) {
        let y_offset = (row as f32 * self.scale.y) as i32;
        let x_offset = (col as f32 * self.scale.y) as i32;
        let color = if value {
            Rgb([255, 255, 255])
        } else {
            Rgb([0, 0, 0])
        };
        draw_text_mut(
            &mut self.img,
            color,
            x_offset,
            y_offset,
            self.scale,
            &self.font,
            &'█'.to_string(),
        );
    }

    /// draw full image from Array2<bool>
    pub fn draw_bools(&mut self, ray: &Array2<bool>) {
        for ((row, col), value) in ray.indexed_iter() {
            self.draw_bool(row, col, *value);
        }
    }

    /// draw full image from Array2<char>
    pub fn draw_chars(&mut self, ray: &Array2<char>) {
        // draw new image row-by-row
        for (rdx, row) in ray.rows().into_iter().enumerate() {
            let full_row: String = row.iter().collect();
            let y_offset = (rdx as f32 * self.scale.y) as i32;
            draw_text_mut(
                &mut self.img,
                Rgb([255, 255, 255]),
                0,
                y_offset,
                self.scale,
                &self.font,
                &full_row,
            );
        }
    }

    // setters & getters

    /// larger alpha -> shorter fade-out per frame
    pub fn set_alpha(&mut self, new_alpha: f32) {
        if new_alpha > 0.0 && new_alpha < 1.0 {
            self.alpha = new_alpha;
        } else {
            println!("err alpha {} out of range (0, 1)", new_alpha);
        }
    }

    pub fn set_framerate(&mut self, new_framerate: u8) {
        self.framerate = new_framerate;
    }

    // frame rendering

    /// save current frame to specific file
    pub fn save_frame(&mut self, file_path: &str) {
        self.img.save(file_path).expect("save failed");
    }

    /// Save current frame to temp dir
    pub fn render_frame(&mut self) {
        let file_path = self.dir.path().join(format!("aoc_{:05}.png", self.fdx));
        self.img.save(file_path).expect("save failed");
        self.fdx += 1;
    }

    /// write framed to animated webp
    pub fn render_webp(&self, file_path: &str) {
        let glob_path = format!("{}", self.dir.path().join("aoc_*.png").display());
        Command::new("ffmpeg")
            .args([
                "-y",
                "-framerate",
                &format!("{}", self.framerate),
                "-pattern_type",
                "glob",
                "-i",
                &glob_path,
                "-vcodec",
                "libwebp",
                // "-quality", // enable lossy (not usually worth it)
                // "10",
                "-lossless", // enable lossless
                "1",
                "-loop", // loop forever
                "0",
                "-preset",
                "drawing",
                file_path,
            ])
            .output()
            .expect("ffmpeg failed");
    }

    /// write frames to animated GIF
    /// for very small images the one pass approach is often better
    pub fn render_gif(&self, file_path: &str) {
        let glob_path = format!("{}", self.dir.path().join("aoc_*.png").display());
        // one pass
        Command::new("ffmpeg")
            .args([
                "-y",
                "-framerate",
                &format!("{}", self.framerate),
                "-pattern_type",
                "glob",
                "-i",
                &glob_path,
                file_path,
            ])
            .output()
            .expect("ffmpeg pass 1 failed");
        // // two pass
        // let pal_path = format!("{}", self.dir.path().join("palette.png").display());
        // // Pass 1
        // Command::new("ffmpeg")
        //     .args([
        //         "-y",
        //         "-pattern_type",
        //         "glob",
        //         "-i",
        //         &glob_path,
        //         "-vf",
        //         "palettegen",
        //         &pal_path,
        //     ])
        //     .output()
        //     .expect("ffmpeg pass 1 failed");

        // // Pass 2
        // Command::new("ffmpeg")
        //     .args([
        //         "-y",
        //         "-framerate",
        //         &format!("{}", self.framerate),
        //         "-pattern_type",
        //         "glob",
        //         "-i",
        //         &glob_path,
        //         "-i",
        //         &pal_path,
        //         "-lavfi",
        //         "paletteuse",
        //         file_path,
        //     ])
        //     .output()
        //     .expect("ffmpeg pass 2 failed");
    }
}
