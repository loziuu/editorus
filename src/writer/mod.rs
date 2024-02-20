use std::io::{stdout, Stdout, Write};

use crossterm::terminal;

use crate::editor::session::{Session, self};

use self::escapes::EscapeSequence;

pub mod escapes;
mod writeable;

type IOResult = std::io::Result<()>;

// TODO: Remove all unwraps
pub fn clear_screen() {
    let mut stdout = stdout();
    EscapeSequence::ClearScreen.execute(&mut stdout).unwrap();
}

pub fn write(session: &mut Session) -> IOResult {
    let cursor = session.cursor();
    let rows = session.rows();
    let mut stdout = stdout();

    if session.is_dirty() {
        EscapeSequence::ClearScreen.execute(&mut stdout)?;
        let (_, lines) = terminal::size().unwrap();

        let num_of_rows = session.rows().len().min(lines as usize);
        EscapeSequence::HideCursor.execute(&mut stdout)?;
        EscapeSequence::MoveCursor(0, 0).execute(&mut stdout)?;
        let data = &session.rows()[..num_of_rows];
        for row in data {
            // TODO: Make it handle better I guess? / no unwrap()
            stdout.write_all(row.data())?;
            EscapeSequence::NewLine.execute(&mut stdout)?;
        }
    }
    EscapeSequence::MoveCursor(cursor.x, cursor.y).execute(&mut stdout)?;
    EscapeSequence::ShowCursor.execute(&mut stdout)?;
    let result = stdout.flush();
    session.mark_clean();

    EscapeSequence::MoveCursor(session.cursor().x, session.cursor().y).execute(&mut stdout)?;
    stdout.flush()?;

    result
}
