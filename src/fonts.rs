use include_dir::{include_dir, Dir, File};
use rusttype::Font;

use std::collections::HashMap;
use toml;

pub const FONTS_DIR: Dir<'_> = include_dir!("assets/fonts");

pub fn get_font_path(font_name: &str) -> &File {
    let filename = FONTS_DIR.get_file("fonts.toml").unwrap();
    let contents = std::str::from_utf8(filename.contents()).unwrap();
    let fonts: HashMap<String, HashMap<String, String>> = toml::from_str(&contents).unwrap();

    if let Some(font_paths) = fonts.get("fonts") {
        if let Some(font_path) = font_paths.get(font_name) {
            let full_path = FONTS_DIR.path().join(font_path);
            println!("Looking for font file at: {:?}", full_path);
            if let Some(file) = FONTS_DIR.get_file(font_path) {
                println!("Font file found at: {:?}", file.path());
                return file;
            }
        }
    }

    println!(
        "Font '{}' not found in the TOML file or directory",
        font_name
    );
    panic!("Font not found");
}

pub fn load_font(font_file: &File) -> Font<'static> {
    let font_data = font_file.contents();
    Font::try_from_vec(font_data.to_vec())
        .ok_or("Error constructing font")
        .unwrap()
}
