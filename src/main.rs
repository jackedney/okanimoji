use okanimoji::generate_ascii_text;
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
        // let text = "おカニ文字";

        if text.eq_ignore_ascii_case("q") {
            break;
        }

        // Prompt for width input
        print!("Enter the width: ");
        stdout().flush()?;
        // let mut width_str = String::new();
        // stdin().read_line(&mut width_str)?;
        // let width: u32 = width_str.trim().parse()?;
        let width = 100;

        // Prompt for font input
        print!("Enter the font: ");
        stdout().flush()?;
        // let mut font = String::new();
        // stdin().read_line(&mut font)?;
        // font = font.trim().to_string();
        let font = "noto-medium";

        // Generate ASCII text
        let ascii_text = generate_ascii_text(&text, &font, width)?;
        println!("ASCII Text:\n{}", ascii_text);
    }

    Ok(())
}
