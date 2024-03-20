use crossterm::terminal;
use editorus::editor::config::Configuration;
use std::io::{stdin, Read};

use editorus::writer;

use editorus::editor::session::Session;

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

pub fn key_check() -> std::io::Result<()> {
    let mut stdin = stdin();
    terminal::enable_raw_mode()?;

    loop {
        let mut buf: [u8; 3] = [0; 3];
        if let Ok(_) = stdin.read(&mut buf) {
            println!("{:?}", buf);
        }
    }
}

fn main() -> std::io::Result<()> {
    run_terminal()
    //    key_check()
}

pub fn run_terminal() -> std::io::Result<()> {
    let config = parse_config()?;

    let (w, h) = terminal::size().unwrap();
    let mut session = Session::with_config(w, h, config);

    let mut args = std::env::args();
    args.next();

    if let Some(file) = args.next() {
        session.open_file(file)?;
    } else {
        println!("No file provided");
        return Ok(());
    }

    let mut stdin = stdin();
    terminal::enable_raw_mode()?;

    loop {
        writer::write(&mut session)?;
        let mut buf: [u8; 3] = [0; 3];
        if let Ok(size) = stdin.read(&mut buf) {
            if buf[0] == 13 {
                session.new_line();
                // TOD Implement new line
            } else if buf[0] == 8 {
                // CTRL + BACKSPACE
                session.backspace();
            } else if buf[0] == 127 {
                // BACKSPACE
                session.backspace();
            } else if buf[0] == 23 {
                // CTRL + W
                session.save_file();
            } else if buf[0] == 21 {
                // CTRL + U
            } else {
                if buf[0] == 24 {
                    // CTRL + X
                    break;
                }
                if buf[0] == 27 {
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
                } else {
                    session.insert(&buf[..size]);
                }
            }
        }
    }

    Ok(())
}

fn parse_config() -> std::io::Result<Configuration> {
    Ok(Configuration {
        show_line_numbers: true,
    })
}
