use std::io::{BufWriter, Stdout, Write};

pub enum EscapeSequence {
    //  TODO: Do we REAAAALLLY need usize here?
    MoveCursor(usize, usize),
    NewLine,
    ClearScreen,
    HideCursor,
    ShowCursor,
}

impl EscapeSequence {
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
            }
            EscapeSequence::HideCursor => {
                stdout.write("[?25l".as_bytes())?;
                Ok(())
            }
            EscapeSequence::ShowCursor => {
                stdout.write("[?25h".as_bytes())?;
                Ok(())
            }
        }
    }

    // TODO: Optimize it later to not allocate on heap
    pub fn sequence(&self) -> Vec<u8> {
        match self {
            EscapeSequence::MoveCursor(x, y) => format!("[{};{}H", y, x).as_bytes().to_vec(),
            EscapeSequence::NewLine => "[1E".as_bytes().to_vec(),
            EscapeSequence::ClearScreen => "[2J".as_bytes().to_vec(),
            EscapeSequence::HideCursor => "[?25l".as_bytes().to_vec(),
            EscapeSequence::ShowCursor => "[?25h".as_bytes().to_vec(),
        }
    }
}
