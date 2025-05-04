use crossterm::{
    cursor,
    execute,
    terminal,
    style::Print
};
use fontdue::Font;
use figlet_rs::FIGfont;
use std::{
    fs::read,
    io::{stdout, Write},
    thread,
    time::Duration,
};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let font_data = read("Roboto-Regular.ttf")?;
    let font = Font::from_bytes(font_data, fontdue::FontSettings::default())
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
    let (metrics, bitmap) = font.rasterize('A', 48.0);

    let standard_font = FIGfont::standard().unwrap();
    let mut stdout = stdout();

    loop {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let hex_time = format!("{:X}", now);
        let figure = standard_font.convert(&hex_time).unwrap();

        // Clear the screen and move cursor to top-left
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // println!("{}", figure

        let width = metrics.width;
        for (i, pixel) in bitmap.iter().enumerate() {
            if i % width == 0 && i != 0 {
                writeln!(stdout)?;
            }

            let ch = match pixel {
                0..=25 => ' ',
                26..=100 => '░',
                101..=200 => '▒',
                _ => '▓',
            };

            write!(stdout, "{}", ch)?;

        }
        thread::sleep(Duration::from_secs(1));
    }
}

