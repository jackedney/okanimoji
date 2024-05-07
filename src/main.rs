use okanimoji::generate_ascii_text;
use std::error::Error;
use std::io::{stdin, stdout, Write};

fn print_kanji(kanji: &str, font: &str, width: u32, offset: u32) {
    let ascii_text = generate_ascii_text(kanji, font, width, offset).unwrap();
    println!("Font: {}", font);
    println!("Width: {}", width);
    println!("Offset: {}", offset);
    println!("{}", ascii_text);
}

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

        for font in &[
            "noto-medium",
            "noto-regular",
            "migu-bold",
            "migu-regular",
            "shippori_mincho-medium",
            "shippori_mincho-semibold",
            "togoshi-gothic",
            "togoshi-mincho",
        ] {
            for width in &[80, 100, 120] {
                for offset in &[0, 1, 2] {
                    print_kanji(&text, font, *width, *offset);
                }
            }
        }
    }

    Ok(())
}
