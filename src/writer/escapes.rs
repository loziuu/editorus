use std::io::{Stdout, Write};

pub enum EscapeSequence {
    MoveCursor(usize, usize),
    NewLine,
    ClearScreen,
    HideCursor,
    ShowCursor,
}

impl EscapeSequence {
    // TODO: Use result :)
    pub fn execute(self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        stdout.write(&[27])?;
        match self {
            EscapeSequence::MoveCursor(x, y) => {
                let sequence = format!("[{};{}H", y, x); // TODO: Can we do it without String?
                stdout.write(sequence.as_bytes())?;
                Ok(())
            }
            EscapeSequence::NewLine => {
                stdout.write("[1E".as_bytes())?;
                Ok(())
            }
            EscapeSequence::ClearScreen => {
                stdout.write("[2J".as_bytes())?;
                Ok(())
            },
            EscapeSequence::HideCursor => {
                stdout.write("[?25l".as_bytes())?;
                Ok(())
            },
            EscapeSequence::ShowCursor => {
                stdout.write("[?25h".as_bytes())?;
                Ok(())
            },
        }
    }
}
