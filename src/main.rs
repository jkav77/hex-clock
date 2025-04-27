use crossterm::{
    cursor,
    execute,
    terminal::{Clear, ClearType},
};
use figlet_rs::FIGfont;
use std::{
    io::{stdout, Result},
    thread,
    time::Duration,
};

fn main() -> Result<()> {
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
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        println!("{}", figure);

        thread::sleep(Duration::from_secs(1));
    }
}

