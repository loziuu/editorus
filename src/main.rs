use crossterm::terminal;
use editor::session::Session;
use std::fs::File;
use std::io::{stdin, Read, Write};
use std::io::{stdout, Stdout};

pub(crate) mod editor;
pub(crate) mod writer;

trait BytesCmp {
    fn bytes_eq(&self, bytes: &[u8]) -> bool;
}

impl BytesCmp for [u8] {
    fn bytes_eq(&self, bytes: &[u8]) -> bool {
        if self.len() != bytes.len() {
            return false;
        }
        for i in 0..self.len() {
            if self[i] != bytes[i] {
                return false;
            }
        }
        true
    }
}

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    let mut stdin = stdin();
    terminal::enable_raw_mode()?;

    let file = File::open("./test.txt")?;
    let mut session = Session::open_file(file)?;

    writer::clear_screen();
    loop {
        writer::write(&mut session)?;

        let mut buf: [u8; 3] = [0; 3];

        if let Ok(size) = stdin.read(&mut buf) {
            if buf[0] == 13 {
                // TODO: Implement new line
            } else if buf[0] == 8 {
                session.backspace();
                return Ok(())
            } else if buf[0] == 127 {
                session.backspace();
            } else if buf[0] == 27 {
                if buf[1..size].bytes_eq("[A".as_bytes()) {
                    session.cursor_up();
                }

                if buf[1..size].bytes_eq("[B".as_bytes()) {
                    session.cursor_down();
                }

                if buf[1..size].bytes_eq("[C".as_bytes()) {
                    session.cursor_right();
                }

                if buf[1..size].bytes_eq("[D".as_bytes()) {
                    session.cursor_left();
                }
            } else if buf[..size].bytes_eq("C".as_bytes()) {
                write!(&stdout, "Exiting")?;
                break;
            } else {
                session.insert(&buf[..size]);
                session.mark_dirty();
            }
        }

        stdout.flush()?;
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
