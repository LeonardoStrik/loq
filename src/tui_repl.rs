use crossterm::terminal::{self, ClearType};
use crossterm::{cursor, QueueableCommand};
use std::io::{self, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

pub fn draw_tui() -> io::Result<()> {
    let mut stdout = stdout();
    let (mut w, mut h) = terminal::size()?;

    let buf = "testie123testie".as_bytes();
    let x = (w - buf.len() as u16) / 2;
    let y = h / 2;

    stdout
        .queue(terminal::Clear(ClearType::All))?
        .queue(cursor::MoveTo(x, y))?;
    stdout.write(buf)?;
    stdout.flush()?;
    sleep(Duration::from_secs(5));
    Ok(())
}
