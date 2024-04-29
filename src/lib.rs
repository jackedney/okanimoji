use image::{ImageBuffer, Rgba, RgbaImage};
use rusttype::{Font, Scale};
use std::error::Error;

const FONT_DICT: &[(&str, &str)] = &[
    ("keifont", "./assets/fonts/keifont.ttf"),
    ("xano", "./assets/fonts/xano.ttf"),
    ("osaka", "./assets/fonts/osaka.ttc"),
    ("migu-regular", "./assets/fonts/migu-regular.ttf"),
    ("migu-bold", "./assets/fonts/migu-bold.ttf"),
    ("togoshi-gothic", "./assets/fonts/togoshi-gothic.ttf"),
    ("shippori-bold", "./assets/fonts/shippori-bold.ttf"),
    ("noto-medium", "./assets/fonts/noto-medium.ttf"),
];

fn load_font(font: &str) -> Result<Font<'static>, Box<dyn Error>> {
    let font_path = FONT_DICT
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

pub fn generate_ascii_text(kanji: &str, font: &str, width: u32) -> Result<String, Box<dyn Error>> {
    let image = generate_kanji_image(kanji, font)?;
    let dynamic_image = image::DynamicImage::ImageRgba8(image);
    Ok(binary_image_to_braille_block_art(&dynamic_image, width))
}

pub fn generate_kanji_image(
    kanji: &str,
    font: &str,
) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let scale = Scale::uniform(64.0);
    let font = load_font(font).unwrap();

    // Calculate the bounding box for each Kanji character
    let glyphs: Vec<_> = font
        .layout(kanji, scale, rusttype::point(0.0, 0.0))
        .collect();
    if glyphs.is_empty() {
        return Err("No glyph found".into());
    }

    let mut image_width: u32 = 0;
    let mut image_height: u32 = 0;

    for glyph in &glyphs {
        let bb = glyph.pixel_bounding_box().unwrap_or_default();
        image_width += bb.width() as u32;
        image_height = image_height.max(bb.height() as u32);
    }

    // Add padding between glyphs
    image_width += (glyphs.len() - 1) as u32 * 10;

    println!(
        "Image width: {}, Image height: {}",
        image_width, image_height
    );

    // Create the image buffer
    let mut image = ImageBuffer::new(image_width, image_height);

    // Render each Kanji character onto the image buffer
    let mut current_x = 0;
    for glyph in &glyphs {
        let bb = glyph.pixel_bounding_box().unwrap_or_default();
        let glyph_height = bb.height() as i32;
        let glyph_width = bb.width() as i32;

        // Calculate the vertical offset to center the glyph
        let vertical_offset = (image_height as i32 - glyph_height) / 2;

        glyph.draw(|x, y, v| {
            let x = current_x + x as u32;
            let y = vertical_offset + y as i32;
            if x >= image_width || y < 0 || y >= image_height as i32 {
                return;
            }
            let color_value = (v * 255.0).ceil() as u8;
            image.put_pixel(
                x,
                y as u32,
                Rgba([color_value, color_value, color_value, 255]),
            );
        });

        current_x += glyph_width as u32 + 10; // Add padding between glyphs
    }

    Ok(image)
}

fn binary_image_to_braille_art(image: &image::DynamicImage, width: u32) -> String {
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
    let height = (width as f32 / aspect_ratio / 2.0).ceil() as u32; // Round up the height to ensure enough space

    let scale_x = image_width as f32 / (width * 2) as f32;
    let scale_y = image_height as f32 / (height * 4) as f32;

    let mut result = String::new();

    for y in 0..height {
        for x in 0..width {
            let mut braille_index = 0;

            for i in 0..4 {
                for j in 0..2 {
                    let pixel_x = ((x * 2 + j) as f32 * scale_x).floor() as u32;
                    let pixel_y = ((y * 4 + i) as f32 * scale_y).floor() as u32;

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

fn binary_image_to_block_art(image: &image::DynamicImage, width: u32) -> String {
    let block_charset: &[&str] = &[
        " ", "▘", "▝", "▀", "▖", "▌", "▞", "▛", "▗", "▚", "▐", "▜", "▄", "▙", "▟", "█",
    ];

    let grayscale_image = image.to_luma8();
    let (image_width, image_height) = grayscale_image.dimensions();

    let aspect_ratio = image_width as f32 / image_height as f32;
    let height = (width as f32 / aspect_ratio / 2.0).ceil() as u32; // Round up the height to ensure enough space

    let scale_x = image_width as f32 / (width * 2) as f32;
    let scale_y = image_height as f32 / (height * 2) as f32;

    let mut result = String::new();

    for y in 0..height {
        for x in 0..width {
            let mut block_index = 0;

            for i in 0..2 {
                for j in 0..2 {
                    let pixel_x = ((x * 2 + j) as f32 * scale_x).floor() as u32;
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

fn binary_image_to_braille_block_art(image: &image::DynamicImage, width: u32) -> String {
    let block_art = binary_image_to_block_art(image, width);
    let braille_art = binary_image_to_braille_art(image, width);

    let mut result = String::new();

    let block_lines: Vec<&str> = block_art.lines().collect();
    let braille_lines: Vec<&str> = braille_art.lines().collect();

    for (block_line, braille_line) in block_lines.iter().zip(braille_lines.iter()) {
        let mut line = String::new();

        for (block_char, braille_char) in block_line.chars().zip(braille_line.chars()) {
            if block_char == ' ' {
                line.push(braille_char);
            } else {
                line.push(block_char);
            }
        }

        result.push_str(&line);
        result.push('\n');
    }

    result
}
