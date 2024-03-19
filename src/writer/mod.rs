use std::io::{stdout, Write};

use crate::editor::session::Session;

use self::escapes::EscapeSequence;

pub mod escapes;
mod writeable;

type IOResult = std::io::Result<()>;

pub fn write(session: &mut Session) -> IOResult {
    let (prev_x, prev_y) = (session.cursor().x, session.cursor().y);
    let mut stdout = stdout();

    if session.is_dirty() {
        EscapeSequence::ClearScreen.execute(&mut stdout)?;
        EscapeSequence::HideCursor.execute(&mut stdout)?;
        EscapeSequence::MoveCursor(0, 0).execute(&mut stdout)?;
        session.display_on(&mut stdout)?;
    }
    EscapeSequence::ShowCursor.execute(&mut stdout)?;
    EscapeSequence::MoveCursor(prev_x, prev_y).execute(&mut stdout)?; 

    stdout.flush()
}
