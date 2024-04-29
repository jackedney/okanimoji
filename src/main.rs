use okanimoji::{generate_ascii_text, generate_kanji_image};
use std::error::Error;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        // Prompt for text input
        print!("Enter the text (or 'q' to quit): ");
        stdout().flush()?;
        let mut text = String::new();
        stdin().read_line(&mut text)?;
        text = text.trim().to_string();

        if text.eq_ignore_ascii_case("q") {
            break;
        }

        // Prompt for width input
        print!("Enter the width: ");
        stdout().flush()?;
        let mut width_str = String::new();
        stdin().read_line(&mut width_str)?;
        let width: u32 = width_str.trim().parse()?;

        // Prompt for font input
        print!("Enter the font: ");
        stdout().flush()?;
        let mut font = String::new();
        stdin().read_line(&mut font)?;
        font = font.trim().to_string();

        // Generate ASCII text
        let ascii_text = generate_ascii_text(&text, &font, width)?;
        println!("ASCII Text:\n{}", ascii_text);

        // Generate ASCII image
        let ascii_image = generate_kanji_image(&text, &font)?;
        let image_filename = format!("{}_output.png", text);
        ascii_image.save(&image_filename)?;
        println!("ASCII Image saved as {}", image_filename);

        println!();
    }

    Ok(())
}

