use super::cursor::ECursor;
use crate::{
    display::display::{Display, Dump, WholeDump},
    rope::rope::Rope,
};
use std::{
    fs::File,
    io::{Read, Stdout},
};

pub struct ERow {
    pub data: Rope,
}

impl ERow {
    fn empty() -> Self {
        Self { data: Rope::new() }
    }

    fn new(data: Rope) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl From<&str> for ERow {
    fn from(value: &str) -> Self {
        Self {
            data: Rope::from(value),
        }
    }
}

pub struct Session {
    // Can it be actually outside?
    // Move to rope once confident
    data: Vec<ERow>,
    display: Display,
    cursor: ECursor,
    dirty: bool,
}

impl Session {
    pub fn new(width: u16, height: u16) -> Self {
        let session = Session {
            data: vec![ERow::empty()], // It's time to move to rope :)
            display: Display::with_dimensions(width, height),
            cursor: ECursor::at_home(),
            dirty: true,
        };
        session
    }

    pub fn open_file(&mut self, mut file: File) -> Result<(), std::io::Error> {
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let rows: Vec<ERow> = content
            .lines()
            .into_iter()
            .map(|row| ERow::from(row))
            .collect();
        self.data = rows;
        self.rebuild_display();
        Ok(())
    }

    pub fn rows(&self) -> &[ERow] {
        &self.data
    }

    pub fn cursor(&self) -> &ECursor {
        &self.cursor
    }

    pub fn cursor_up(&mut self) {
        self.cursor.up();
        if self.cursor.x > self.data[self.cursor.y - 1].len() {
            self.cursor.x = self.data[self.cursor.y - 1].len();
        }

        if self.cursor.x == 0 {
            self.cursor.x = 1;
        }
    }

    pub fn cursor_down(&mut self) {
        if self.cursor.y != self.data.len() {
            self.cursor.down();
        }
        if self.cursor.x > self.data[self.cursor.y - 1].len() {
            self.cursor.x = self.data[self.cursor.y - 1].len();
        }
    }

    pub fn cursor_left(&mut self) {
        if self.cursor.x == 1 {
            if self.cursor.y != 1 {
                self.cursor.up();
                self.cursor.x = self.data[self.cursor.y - 1].len() + 1; // (cursor.x = 1) == data[0]
            }
        }
        self.cursor.left();
    }

    pub fn cursor_right(&mut self) {
        if self.cursor.x <= self.data[self.cursor.y - 1].len() {
            self.cursor.right();
        } else {
            self.cursor.down();
            self.cursor.move_to_line_beginning();
        }
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_dirty(&mut self) {
        self.rebuild_display();
        self.dirty = true;
    }

    fn rebuild_display(&mut self) {
        self.display.refresh(&self.data);
    }

    // TODO: Refactor maybe to use some commands?
    // TODO: Make it work at the end of the line
    pub fn insert(&mut self, data: &[u8]) {
        let row = &mut self.data[self.cursor.y - 1];
        let data = std::str::from_utf8(data).unwrap();
        row.data.insert(self.cursor.x - 1, data);

        for _ in 0..data.len() {
            self.cursor.right();
        }

        self.mark_dirty();
    }

    // TODO: Refactor maybe to use some commands?
    pub fn backspace(&mut self) {
        if self.cursor.x == 1 && self.cursor.y == 1 {
            return;
        }
        if self.cursor.at_start() {
            // We need to concat ropes
            let prev_row = self.data.remove(self.cursor.y - 2);
            let curr_row = self.data.remove(self.cursor.y - 2);
            let x_final_position = prev_row.len();
            let concat = prev_row.data.concat(curr_row.data);
            self.data.insert(self.cursor.y - 2, ERow::new(concat));
            self.cursor.up();
            self.cursor.x = x_final_position + 1;
        } else {
            self.cursor.left();
            let row = &mut self.data[self.cursor.y - 1];
            row.data.remove_at(self.cursor.x() - 1);
        }
        self.mark_dirty();
    }

    // This should actually change data, but rather display... right?
    pub fn new_line(&mut self) {
        let current_row = &self.data[self.cursor.y - 1];

        if self.cursor.x() - 1 != current_row.data.len() {
            let row = &mut self.data[self.cursor.y - 1];
            let (curr, next) = row.data.split_at(self.cursor.x() - 1);
            row.data = curr;
            self.data.insert(self.cursor.y, ERow::new(next));
        } else {
            self.data.insert(self.cursor.y, ERow::empty());
        }

        self.cursor.down();
        self.cursor.move_to_line_beginning();

        self.mark_dirty();
    }

    pub(crate) fn display_on(&mut self, stdout: &mut Stdout) -> std::io::Result<()> {
        WholeDump::new(&self.display).dump_to(stdout);
        self.mark_clean();
        Ok(())
    }
}
