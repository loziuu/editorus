use std::io::{stdout, Write};

use crossterm::terminal;

use crate::editor::session::Session;

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
    let mut stdout = stdout();

    if session.is_dirty() {
        EscapeSequence::ClearScreen.execute(&mut stdout)?;
        let lines = session.display_height(); 
        EscapeSequence::HideCursor.execute(&mut stdout)?;
        EscapeSequence::MoveCursor(0, 0).execute(&mut stdout)?;
        session.display_on(&mut stdout)?;

    }
    EscapeSequence::MoveCursor(cursor.x, cursor.y).execute(&mut stdout)?;
    EscapeSequence::ShowCursor.execute(&mut stdout)?;
    let result = stdout.flush();
    session.mark_clean();

    EscapeSequence::MoveCursor(session.cursor().x, session.cursor().y).execute(&mut stdout)?;
    stdout.flush()?;

    result
}
