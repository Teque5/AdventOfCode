/// Image Rendering
use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use ndarray::Array2;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::process::Command;
use tempfile::{tempdir, TempDir};

/// Trait for converting various types to boolean for drawing
pub trait ToBool {
    fn to_bool(&self) -> bool;
}

impl ToBool for bool {
    fn to_bool(&self) -> bool {
        *self
    }
}

macro_rules! impl_to_bool_for_integers {
    ($($numeric_type:ty),*) => {
        $(
            impl ToBool for $numeric_type {
                fn to_bool(&self) -> bool {
                    *self != 0
                }
            }
        )*
    };
}

impl_to_bool_for_integers!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);

macro_rules! impl_to_bool_for_floats {
    ($($float_type:ty),*) => {
        $(
            impl ToBool for $float_type {
                fn to_bool(&self) -> bool {
                    *self != 0.0
                }
            }
        )*
    };
}

impl_to_bool_for_floats!(f32, f64);

/// convert all chars not in ' .' to true
impl ToBool for char {
    fn to_bool(&self) -> bool {
        *self != ' ' && *self != '.'
    }
}

/// Font name to download and cache
const UNSCII_FONT: &str = "unscii-8";
/// URL for the unscii font
const UNSCII_FONT_URL: &str = "https://github.com/viznut/unscii/raw/refs/heads/main/fontfiles";

/// Get the font cache directory
fn get_cache_dir() -> std::io::Result<PathBuf> {
    let home = std::env::var("HOME").expect("HOME environment variable not set");
    let cache_dir = PathBuf::from(home).join(".cache").join("aoc-fonts");
    create_dir_all(&cache_dir)?;
    Ok(cache_dir)
}

/// Get font data, downloading and caching if necessary
pub fn get_cached_font() -> std::io::Result<Vec<u8>> {
    let cache_dir = get_cache_dir()?;
    let cache_path = cache_dir.join(format!("{}.otf", UNSCII_FONT));
    // check if font is already cached
    if cache_path.exists() {
        return std::fs::read(&cache_path);
    }
    // download the font using reqwest
    println!("Downloading {} font from GitHub...", UNSCII_FONT);
    let data = download_unscii(&cache_path).expect("Failed to download font");
    println!("Font cached to: {:?}", cache_path);
    Ok(data)
}

/// Download font (blocking)
fn download_unscii(font_path: &PathBuf) -> Option<Vec<u8>> {
    let font_filename = format!("{}.otf", UNSCII_FONT);
    let font_url = format!("{}/{}", UNSCII_FONT_URL, font_filename);
    let client = reqwest::blocking::Client::new();
    let response = client.get(&font_url).send().ok()?;
    if !response.status().is_success() {
        return None;
    }
    let font_data = response.bytes().ok()?.to_vec();
    // Save to cache
    std::fs::write(font_path, &font_data).ok()?;
    Some(font_data)
}

pub struct Image {
    img: RgbImage,
    font: FontRef<'static>,
    scale: PxScale,
    dir: TempDir,
    rows: usize,
    cols: usize,
    fdx: usize, // frame index
    frameskip: usize,
    alpha: f32,    // fade alpha
    framerate: u8, // gif framerate
}

/// Very often in AoC we have an Array2<_> where we want to visualize the changes
/// ## Example (not doctest)
/// let mut img = aoc::Image::new(10, 20);
/// for _ in 0..10 {
///     img.draw_chars(&some_2d_array);
///     img.render_frame();
///     img.fade();
/// }
/// img.render_gif("out.gif");
impl Image {
    /// create new image surface
    pub fn new(rows: usize, cols: usize) -> Self {
        // font needs to be monospace - get cached version
        let font_data = get_cached_font().expect("Failed to get font");
        // leak the font data to get a 'static reference for FontRef
        let static_font_data = Box::leak(font_data.into_boxed_slice());
        let font = FontRef::try_from_slice(static_font_data).unwrap();
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
            frameskip: 1,
            alpha: 0.05,
            framerate: 15,
        }
        // TODO: check that ffmpeg is installed
    }

    /// clear image and redefine character size
    /// 8 (default) is pixel-perfect for text
    /// 4 is barely resolvable
    /// 1 is for tiny pixel plotting using draw_bool
    pub fn set_scale(&mut self, size_px: usize) {
        self.scale.x = size_px as f32;
        self.scale.y = size_px as f32;
        self.img = RgbImage::new((size_px * self.cols) as u32, (size_px * self.rows) as u32);
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

    /// print img info
    pub fn info(&self) {
        // fdx, size, alpha, framerate, tempdir path
        println!(
            "Image: {}x{} px, {} frames, alpha {:.2}, framerate {} fps, tempdir {:?}",
            self.img.width(),
            self.img.height(),
            self.fdx,
            self.alpha,
            self.framerate,
            self.dir.path()
        );
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

    /// draw full image from Array2<T> as booleans
    pub fn draw_bools<T: ToBool>(&mut self, ray: &Array2<T>) {
        for ((row, col), value) in ray.indexed_iter() {
            if value.to_bool() {
                self.draw_bool(row, col, true);
            }
        }
    }

    /// draw full image from Array2<T>
    pub fn draw_chars<T: std::fmt::Display>(&mut self, ray: &Array2<T>) {
        // draw new image row-by-row
        for (rdx, row) in ray.rows().into_iter().enumerate() {
            let full_row: String = row.iter().map(|x| x.to_string()).collect();
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

    pub fn set_frameskip(&mut self, new_frameskip: usize) {
        self.frameskip = new_frameskip;
    }

    // frame rendering

    /// save current frame to specific file
    pub fn save_frame(&mut self, file_path: &str) {
        self.img.save(file_path).expect("save failed");
    }

    /// Save current frame to temp dir
    pub fn render_frame(&mut self) {
        // check for frameskip
        if self.fdx % self.frameskip == 0 {
            let file_path = self.dir.path().join(format!("aoc_{:08}.png", self.fdx));
            self.img.save(file_path).expect("save failed");
        }
        self.fdx += 1;
    }

    /// write frames to animated webp
    pub fn render_webp(&self, file_path: &str) {
        let glob_path = format!("{}", self.dir.path().join("aoc_*.png").display());
        let num_images = (self.fdx + self.frameskip - 1) / self.frameskip;
        println!("Image: rendering {} imgs to {} ", num_images, file_path);
        Command::new("ffmpeg")
            .args([
                "-y",
                "-framerate",
                &format!("{}", self.framerate),
                "-pattern_type",
                "glob",
                "-i",
                &glob_path,
                "-vf",
                "format=yuv420p",
                "-vcodec",
                // new 2025 encoder
                "libwebp_anim",
                "-lossless",
                "0",
                // 6 takes way too long
                "-compression_level",
                "5",
                // loop forever
                "-loop",
                "0",
                file_path,
            ])
            .output()
            .expect("ffmpeg failed");
        Self::print_file_size(file_path);
    }

    /// write frames to animated GIF
    /// for very small images the one pass approach is often better
    pub fn render_gif(&self, file_path: &str) {
        let num_images = (self.fdx + self.frameskip - 1) / self.frameskip;
        println!("Image: rendering {} imgs to {} ", num_images, file_path);
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
            .expect("ffmpeg failed");
        Self::print_file_size(file_path);
    }

    pub fn render_gif_two_pass(&self, file_path: &str) {
        let num_images = (self.fdx + self.frameskip - 1) / self.frameskip;
        println!("Image: rendering {} imgs to {} ", num_images, file_path);
        let glob_path = format!("{}", self.dir.path().join("aoc_*.png").display());
        let palette_path = self.dir.path().join("palette.png");
        // first pass: generate palette
        Command::new("ffmpeg")
            .args([
                "-y",
                "-framerate",
                &format!("{}", self.framerate),
                "-pattern_type",
                "glob",
                "-i",
                &glob_path,
                "-vf",
                "palettegen",
                "-y",
                &format!("{}", palette_path.display()),
            ])
            .output()
            .expect("ffmpeg failed");
        // second pass: generate gif using palette
        Command::new("ffmpeg")
            .args([
                "-y",
                "-framerate",
                &format!("{}", self.framerate),
                "-pattern_type",
                "glob",
                "-i",
                &glob_path,
                "-i",
                &format!("{}", palette_path.display()),
                "-lavfi",
                "paletteuse",
                file_path,
            ])
            .output()
            .expect("ffmpeg failed");
        Self::print_file_size(file_path);
    }

    fn print_file_size(file_path: &str) {
        if let Ok(metadata) = std::fs::metadata(file_path) {
            let size_kb = metadata.len() / 1024;
            println!("Image: wrote {:.1} KB", size_kb as f64);
        }
    }
}
