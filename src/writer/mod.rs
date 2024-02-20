use std::{
    io::{stdout, Result, Write},
    time::Instant,
};

use crossterm::terminal;

use crate::editor::session::Session;

use self::escapes::EscapeSequence;

mod escapes;

// TODO: Remove all unwraps
pub(crate) fn clear_screen() {
    let mut stdout = stdout();
    EscapeSequence::ClearScreen.execute(&mut stdout).unwrap();
}

pub(crate) fn write(session: &mut Session) -> Result<()> {
    let start = Instant::now();
    let cursor = session.cursor();
    let rows = session.rows();
    let mut stdout = stdout();

    //    if session.is_dirty() {
    let (_, lines) = terminal::size().unwrap();

    let num_of_rows = rows.len().min(lines as usize);
    EscapeSequence::HideCursor.execute(&mut stdout)?;
    EscapeSequence::MoveCursor(0, 0).execute(&mut stdout)?;
    for i in 0..num_of_rows {
        // TODO: Make it handle better I guess? / no unwrap()
        stdout.write_all(&rows[i].data())?;
        EscapeSequence::NewLine.execute(&mut stdout)?;
    }

    let msg = format!("Took {} ms.", start.elapsed().as_millis());
    EscapeSequence::MoveCursor(0, lines as usize).execute(&mut stdout)?;
    stdout.write_all(&msg.as_bytes()).unwrap();
    //   }
    EscapeSequence::MoveCursor(cursor.x, cursor.y).execute(&mut stdout)?;
    EscapeSequence::ShowCursor.execute(&mut stdout)?;
    let result = stdout.flush();
    session.mark_clean();
    result
}
