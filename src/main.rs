use std::collections::HashMap;
use crossterm::{cursor, execute, terminal};
use fontdue::Font;
use std::{fs::read, io::{stdout, Write}, thread, time::Duration};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let font_data = read("DejaVuSansMono.ttf")?;
    let font = Font::from_bytes(font_data, fontdue::FontSettings::default())
        .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;
    let font_size = 48.0;
    let hex_chars = "0123456789abcdef".chars();

    // Pre-rasterize all hex characters
    let mut glyphs: HashMap<char, (fontdue::Metrics, Vec<u8>)> = HashMap::new();
    let mut max_ascent = 0;
    let mut max_descent = 0;

    for c in hex_chars {
        let (metrics, bitmap) = font.rasterize(c, font_size);
        max_ascent = max_ascent.max(metrics.height as i32 + metrics.ymin);
        max_descent = max_descent.max(-metrics.ymin);
        glyphs.insert(c, (metrics, bitmap));
    }

    let mut stdout = stdout();

    loop {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let hex_time = format!("{:x}", now);

        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        let total_height = max_ascent + max_descent;
        let rows = (total_height as usize + 1) / 2; // merge two bitmap rows into 1 terminal row
                                                    //
        for row in 0..rows {
            let upper_y = row * 2;
            let lower_y = upper_y + 1;

            for ch in hex_time.chars() {
                if let Some((metrics, bitmap)) = glyphs.get(&ch) {
                    let width = metrics.width;
                    let height = metrics.height;
                    let baseline_offset = max_ascent - (height as i32 + metrics.ymin);

                    let pixel_at = |y: usize, x: usize| -> u8 {
                        if y < height && x < width {
                            bitmap[y * width + x]
                        } else {
                            0
                        }
                    };

                    for x in 0..width {
                        let uy = upper_y as i32 - baseline_offset;
                        let ly = lower_y as i32 - baseline_offset;

                        let upper = if uy >= 0 && uy < height as i32 {
                            pixel_at(uy as usize, x)
                        } else {
                            0 
                        };

                        let lower = if ly >= 0 && ly < height as i32 {
                            pixel_at(ly as usize, x)
                        } else {
                            0
                        };

                        let ch = match (upper, lower) {
                            (0..=25, 0..=25) => ' ',
                            (_, 0..=25) => '▀',
                            (0..=25, _) => '▄',
                            _ => '█',
                        };
                        write!(stdout, "{}", ch)?;
                    }
                }
            }
            writeln!(stdout)?;
        }

        thread::sleep(Duration::from_secs(1));
    }
}

