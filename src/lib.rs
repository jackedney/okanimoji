use image::{Rgba, RgbaImage};
use rusttype::{Font, Scale};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use terminal_size::{terminal_size, Width};

const FONT_DIR: &str = "./assets/fonts";

fn generate_font_dict() -> HashMap<String, String> {
    fs::read_dir(FONT_DIR)
        .expect("Failed to read font directory")
        .flat_map(|entry| {
            let path = entry.expect("Failed to get entry path").path();
            if path.is_dir() {
                fs::read_dir(path).expect("Failed to read subdirectory")
            } else {
                fs::read_dir(path).unwrap_or_else(|_| {
                    fs::read_dir(FONT_DIR).expect("Failed to read font directory")
                })
            }
        })
        .filter_map(|entry| {
            let path = entry.expect("Failed to get entry path").path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "ttf") {
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(|font_name| (font_name.to_string(), path.to_str().unwrap().to_string()))
            } else {
                None
            }
        })
        .collect()
}

fn load_font(
    font: &str,
    font_dict: HashMap<String, String>,
) -> Result<Font<'static>, Box<dyn Error>> {
    let font_path = font_dict
        .iter()
        .find(|(name, _)| name == &font)
        .map(|(_, path)| path);

    match font_path {
        Some(path) => {
            let font_data = std::fs::read(path)?;
            let font = Font::try_from_vec(font_data).ok_or("Error constructing font")?;
            Ok(font)
        }
        None => Err(format!("Font '{}' not found", font).into()),
    }
}

pub fn generate_kanji_image(
    kanji: &str,
    font: &str,
) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    // Load the OpenType font from file
    let scale = Scale::uniform(64.0);
    let font_dict = generate_font_dict();
    let font = load_font(font, font_dict).unwrap();

    // Measure the dimensions of the text
    let v_metrics = font.v_metrics(scale);

    let glyphs: Vec<_> = font
        .layout(kanji, scale, rusttype::point(0.0, 0.0))
        .collect();

    let text_width = glyphs
        .last()
        .map(|g| {
            let mut width = g.position().x + g.unpositioned().h_metrics().advance_width;
            for (prev, curr) in glyphs.iter().zip(glyphs.iter().skip(1)) {
                width += font.pair_kerning(scale, prev.id(), curr.id());
            }
            width
        })
        .unwrap_or(0.0);
    let text_height = v_metrics.ascent - v_metrics.descent;

    // Create a new image buffer
    let mut image = RgbaImage::new(text_width.ceil() as u32, text_height.ceil() as u32);

    // Draw the glyphs onto the image
    let mut x = 0.0;
    for (prev, curr) in std::iter::once(None)
        .chain(glyphs.iter().map(Some))
        .zip(glyphs.iter())
    {
        let y =
            curr.position().y + v_metrics.ascent + curr.pixel_bounding_box().unwrap().min.y as f32;
        curr.draw(|gx, gy, gv| {
            let gx = x + gx as f32;
            let gy = y + gy as f32;
            if gy >= 0.0 && gy < image.height() as f32 {
                let color_value = (gv * 255.0).ceil() as u8;
                image.put_pixel(
                    gx as u32,
                    gy as u32,
                    Rgba([color_value, color_value, color_value, 255]),
                );
            }
        });
        x += curr.unpositioned().h_metrics().advance_width;
        if let Some(prev) = prev {
            x += font.pair_kerning(scale, prev.id(), curr.id());
        }
    }

    Ok(image)
}

pub fn generate_ascii_text(
    text: &str,
    font: &str,
    min_height: u32,
    braille_offset: u32,
) -> Result<String, Box<dyn Error>> {
    let image = generate_kanji_image(text, font)?;
    let dynamic_image = image::DynamicImage::ImageRgba8(image);
    let terminal_width = terminal_size().map(|(Width(w), _)| w as u32).unwrap_or(80);
    Ok(binary_image_to_braille_block_art(
        &dynamic_image,
        terminal_width,
        min_height,
        braille_offset,
    ))
}

fn binary_image_to_braille_art(
    image: &image::DynamicImage,
    max_width: u32,
    min_height: u32,
) -> String {
    let charset: &[&str] = &[
        " ", "⠁", "⠂", "⠃", "⠄", "⠅", "⠆", "⠇", "⠈", "⠉", "⠊", "⠋", "⠌", "⠍", "⠎", "⠏", "⠐", "⠑",
        "⠒", "⠓", "⠔", "⠕", "⠖", "⠗", "⠘", "⠙", "⠚", "⠛", "⠜", "⠝", "⠞", "⠟", "⠠", "⠡", "⠢", "⠣",
        "⠤", "⠥", "⠦", "⠧", "⠨", "⠩", "⠪", "⠫", "⠬", "⠭", "⠮", "⠯", "⠰", "⠱", "⠲", "⠳", "⠴", "⠵",
        "⠶", "⠷", "⠸", "⠹", "⠺", "⠻", "⠼", "⠽", "⠾", "⠿", "⡀", "⡁", "⡂", "⡃", "⡄", "⡅", "⡆", "⡇",
        "⡈", "⡉", "⡊", "⡋", "⡌", "⡍", "⡎", "⡏", "⡐", "⡑", "⡒", "⡓", "⡔", "⡕", "⡖", "⡗", "⡘", "⡙",
        "⡚", "⡛", "⡜", "⡝", "⡞", "⡟", "⡠", "⡡", "⡢", "⡣", "⡤", "⡥", "⡦", "⡧", "⡨", "⡩", "⡪", "⡫",
        "⡬", "⡭", "⡮", "⡯", "⡰", "⡱", "⡲", "⡳", "⡴", "⡵", "⡶", "⡷", "⡸", "⡹", "⡺", "⡻", "⡼", "⡽",
        "⡾", "⡿", "⢀", "⢁", "⢂", "⢃", "⢄", "⢅", "⢆", "⢇", "⢈", "⢉", "⢊", "⢋", "⢌", "⢍", "⢎", "⢏",
        "⢐", "⢑", "⢒", "⢓", "⢔", "⢕", "⢖", "⢗", "⢘", "⢙", "⢚", "⢛", "⢜", "⢝", "⢞", "⢟", "⢠", "⢡",
        "⢢", "⢣", "⢤", "⢥", "⢦", "⢧", "⢨", "⢩", "⢪", "⢫", "⢬", "⢭", "⢮", "⢯", "⢰", "⢱", "⢲", "⢳",
        "⢴", "⢵", "⢶", "⢷", "⢸", "⢹", "⢺", "⢻", "⢼", "⢽", "⢾", "⢿", "⣀", "⣁", "⣂", "⣃", "⣄", "⣅",
        "⣆", "⣇", "⣈", "⣉", "⣊", "⣋", "⣌", "⣍", "⣎", "⣏", "⣐", "⣑", "⣒", "⣓", "⣔", "⣕", "⣖", "⣗",
        "⣘", "⣙", "⣚", "⣛", "⣜", "⣝", "⣞", "⣟", "⣠", "⣡", "⣢", "⣣", "⣤", "⣥", "⣦", "⣧", "⣨", "⣩",
        "⣪", "⣫", "⣬", "⣭", "⣮", "⣯", "⣰", "⣱", "⣲", "⣳", "⣴", "⣵", "⣶", "⣷", "⣸", "⣹", "⣺", "⣻",
        "⣼", "⣽", "⣾", "⣿",
    ];

    let grayscale_image = image.to_luma8();
    let (image_width, image_height) = grayscale_image.dimensions();

    let aspect_ratio = image_width as f32 / image_height as f32;

    let width = std::cmp::min(max_width, (aspect_ratio * min_height as f32).ceil() as u32);
    let height = (width as f32 / aspect_ratio).ceil() as u32;

    let scale_x = image_width as f32 / width as f32;
    let scale_y = image_height as f32 / height as f32;

    let mut result = String::new();

    for y in 0..(height / 2) {
        for x in 0..width {
            let mut braille_index = 0;
            for i in 0..2 {
                for j in 0..2 {
                    let pixel_x = ((x + j) as f32 * scale_x).floor() as u32;
                    let pixel_y = ((y * 2 + i) as f32 * scale_y).floor() as u32;

                    if pixel_y < image_height
                        && pixel_x < image_width
                        && grayscale_image.get_pixel(pixel_x, pixel_y)[0] > 170
                    {
                        braille_index |= 1 << (i * 2 + j);
                    }
                }
            }

            result.push_str(charset[braille_index]);
        }
        result.push('\n');
    }

    result
}

fn binary_image_to_block_art(
    image: &image::DynamicImage,
    max_width: u32,
    min_height: u32,
) -> String {
    let block_charset: &[&str] = &[
        " ", "▘", "▝", "▀", "▖", "▌", "▞", "▛", "▗", "▚", "▐", "▜", "▄", "▙", "▟", "█",
    ];

    let grayscale_image = image.to_luma8();
    let (image_width, image_height) = grayscale_image.dimensions();

    let aspect_ratio = image_width as f32 / image_height as f32;

    let width = std::cmp::min(max_width, (aspect_ratio * min_height as f32).ceil() as u32);
    let height = (width as f32 / aspect_ratio).ceil() as u32;

    let scale_x = image_width as f32 / width as f32;
    let scale_y = image_height as f32 / height as f32;

    let mut result = String::new();

    for y in 0..(height / 2) {
        for x in 0..width {
            let mut block_index = 0;

            for i in 0..2 {
                for j in 0..2 {
                    let pixel_x = ((x + j) as f32 * scale_x).floor() as u32;
                    let pixel_y = ((y * 2 + i) as f32 * scale_y).floor() as u32;

                    if pixel_y < image_height
                        && pixel_x < image_width
                        && grayscale_image.get_pixel(pixel_x, pixel_y)[0] > 200
                    {
                        block_index |= 1 << (i * 2 + j);
                    }
                }
            }

            result.push_str(block_charset[block_index]);
        }
        result.push('\n');
    }

    result
}

fn binary_image_to_braille_block_art(
    image: &image::DynamicImage,
    mut max_width: u32,
    min_height: u32,
    shadow_offset: u32,
) -> String {
    max_width = max_width - shadow_offset;
    let block_art = binary_image_to_block_art(image, max_width, min_height);
    let braille_art = binary_image_to_braille_art(image, max_width, min_height);

    let mut result = String::new();

    let block_lines: Vec<&str> = block_art.lines().collect();
    let braille_lines: Vec<&str> = braille_art.lines().collect();
    let mut braille_char: char;
    let mut block_char: char;
    let row_length = block_lines[0].len() + shadow_offset as usize;
    let num_rows = block_lines.len() + shadow_offset as usize;

    for i in 0..num_rows {
        for j in 0..row_length {
            if i >= block_lines.len() || j >= block_lines[i].len() {
                block_char = ' ';
            } else {
                block_char = block_lines[i].chars().nth(j).unwrap_or(' ');
            }
            if i >= shadow_offset as usize && j >= shadow_offset as usize {
                braille_char = braille_lines[i - shadow_offset as usize]
                    .chars()
                    .nth(j - shadow_offset as usize)
                    .unwrap_or(' ');
            } else {
                braille_char = ' ';
            }

            if block_char == ' ' {
                result.push(braille_char);
            } else {
                result.push(block_char);
            }
        }
        result.push('\n');
    }
    result
}
