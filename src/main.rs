use crossterm::{terminal, ExecutableCommand};
use editorus::rope::rope::Rope;
use editorus::rope::traverser::Traverser;
use editorus::writer::escapes::EscapeSequence;
use std::fs::File;
use std::io::stdout;
use std::io::{stdin, Read, Write};
use std::sync::Arc;

use editorus::{rope, writer};

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

//static PHRASE: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ";

//static PHRASE: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ";

static PHRASE: &'static str = "abcdefghijklmnopqr";

fn run_rope() -> std::io::Result<()> {
    let mut rope = Rope::new();
    let mut stdin = stdin();
    let mut traverser = Arc::new(Traverser::new(&rope));

    terminal::enable_raw_mode()?;
    EscapeSequence::MoveCursor(0, 0).execute(&mut stdout())?;
    EscapeSequence::ClearScreen.execute(&mut stdout())?;

    loop {
        let mut buf: [u8; 3] = [0; 3];

        if let Ok(size) = stdin.read(&mut buf) {
            EscapeSequence::MoveCursor(0, 0).execute(&mut stdout())?;
            EscapeSequence::ClearScreen.execute(&mut stdout())?;
            if buf[0] == 13 {
                rope.append(PHRASE);
                traverser = Arc::new(Traverser::new(&rope));
            } else if buf[0] == 127 {
                println!("Rebalancing. Moving back to root.");
                rope.rebalance();
                traverser = Arc::new(Traverser::new(&rope));
            } else if buf[0] == 27 {
                if buf[1..size].bytes_eq("[A".as_bytes()) {
                    traverser = traverser.go_back();
                }

                if buf[1..size].bytes_eq("[B".as_bytes()) {
                    traverser.current();
                }

                if buf[1..size].bytes_eq("[C".as_bytes()) {
                    traverser = traverser.go_right();
                }

                if buf[1..size].bytes_eq("[D".as_bytes()) {
                    traverser = traverser.go_left();
                }
            } else if buf[..size].bytes_eq("C".as_bytes()) {
                break;
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    //run_terminal()
    run_rope()
}

pub fn run_terminal() -> std::io::Result<()> {
    setup_logger();
    let mut stdout = stdout();
    let mut stdin = stdin();
    terminal::enable_raw_mode()?;

    let file = File::open("./test.txt")?;
    let mut session = Session::open_file(file)?;

    writer::clear_screen();
    writer::write(&mut session)?;
    loop {
        let mut buf: [u8; 3] = [0; 3];

        if let Ok(size) = stdin.read(&mut buf) {
            if buf[0] == 13 {
                session.new_line();
                // TOD Implement new line
            } else if buf[0] == 8 {
                session.backspace();
                return Ok(());
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
            writer::write(&mut session)?;
        }

        stdout.flush()?;
    }

    terminal::disable_raw_mode()?;
    Ok(())
}

// TODO: Tidy this shit up
fn setup_logger() {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("output.log").unwrap())
        .apply()
        .unwrap()
}
