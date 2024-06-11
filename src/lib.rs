use image::{Rgba, RgbaImage};
use rusttype::Scale;
use std::error::Error;
use terminal_size::{terminal_size, Width};

mod fonts;
use fonts::{get_font_path, load_font};

pub fn generate_kanji_image(kanji: &str, font: &str) -> Result<RgbaImage, Box<dyn Error>> {
    // Load the OpenType font from file
    let scale = Scale::uniform(64.0);
    let font_path = get_font_path(font);
    let font = load_font(&font_path);

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

pub fn generate_ascii_text(text: &str, font: &str, min_height: u32, braille_offset: u32) -> String {
    let image = generate_kanji_image(text, font);
    let dynamic_image = image::DynamicImage::ImageRgba8(image.unwrap());
    let terminal_width = terminal_size().map(|(Width(w), _)| w as u32).unwrap();
    binary_image_to_braille_block_art(&dynamic_image, terminal_width, min_height, braille_offset)
}

fn binary_image_to_braille_art(
    image: &image::DynamicImage,
    max_width: u32,
    min_height: u32,
) -> Vec<Vec<char>> {
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

    let mut result: Vec<Vec<char>> = Vec::new();

    for y in 0..(height / 2) {
        let mut row: Vec<char> = Vec::new();
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
            row.push(charset[braille_index].chars().nth(0).unwrap());
        }
        result.push(row);
    }
    result
}

fn binary_image_to_block_art(
    image: &image::DynamicImage,
    max_width: u32,
    min_height: u32,
) -> Vec<Vec<char>> {
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

    let mut result = Vec::new();

    for y in 0..(height / 2) {
        let mut row: Vec<char> = Vec::new();
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

            row.push(block_charset[block_index].chars().nth(0).unwrap());
        }
        result.push(row);
    }
    result
}

fn binary_image_to_braille_block_art(
    image: &image::DynamicImage,
    mut max_width: u32,
    min_height: u32,
    shadow_offset: u32,
) -> String {
    max_width = (max_width - shadow_offset) - 5;
    let block_art = binary_image_to_block_art(image, max_width, min_height);
    let braille_art = binary_image_to_braille_art(image, max_width, min_height);
    let mut result = String::new();
    let block_lines: Vec<Vec<char>> = block_art;
    let braille_lines: Vec<Vec<char>> = braille_art;
    let mut braille_char: char;
    let mut block_char: char;
    let row_length = block_lines[0].len() + shadow_offset as usize;
    let num_rows = block_lines.len() + shadow_offset as usize;

    for i in 0..num_rows {
        for j in 0..row_length {
            if i >= block_lines.len() || j >= block_lines[i].len() {
                block_char = ' ';
            } else {
                block_char = block_lines[i][j];
            }
            if i > shadow_offset as usize && j > shadow_offset as usize {
                braille_char = braille_lines[i - shadow_offset as usize][j - shadow_offset as usize]
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

pub fn generate_ascii_image(
    image: &image::DynamicImage,
    max_width: u32,
    min_height: u32,
    shadow_offset: u32,
) -> String {
    return binary_image_to_braille_block_art(image, max_width, min_height, shadow_offset);
}
