use super::cursor::ECursor;
use std::{fs::File, io::Read};

pub struct ERow {
    data: Vec<u8>,
}

impl ERow {
    fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    fn empty(allocated: usize) -> Self {
        Self {
            data: vec![0; allocated],
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub struct Session {
    // Can it be actually outside?
    data: Vec<ERow>,
    cursor: ECursor,
    dirty: bool,
}

impl Session {
    pub fn open_file(mut file: File) -> Result<Self, std::io::Error> {
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let rows: Vec<ERow> = content
            .lines()
            .into_iter()
            .map(|line| line.as_bytes())
            .map(|row| ERow::new(row.to_vec()))
            .collect();
        let session = Session {
            data: rows,
            cursor: ECursor::at_home(),
            dirty: true,
        };
        Ok(session)
    }

    pub fn rows(&self) -> &[ERow] {
        self.data.as_ref()
    }

    pub fn cursor(&self) -> &ECursor {
        &self.cursor
    }

    pub fn cursor_up(&mut self) {
        self.cursor.up();
        if self.cursor.x > self.data[self.cursor.y - 1].len() {
            self.cursor.x = self.data[self.cursor.y - 1].len();
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
        self.mark_dirty();
    }

    pub fn cursor_right(&mut self) {
        if self.cursor.x < self.data[self.cursor.y - 1].len() {
            self.cursor.right();
        } else {
            self.cursor.down();
            self.cursor.move_to_line_beginning();
        }
        self.mark_dirty();
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    // TODO: Refactor maybe to use some commands?
    // TODO: Make it work at the end of the line
    pub fn insert(&mut self, data: &[u8]) {
        let row = &mut self.data[self.cursor.y - 1];

        for bytes in data {
            row.data.insert(self.cursor.x - 1, *bytes);
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
            let data = self.data[self.cursor.y - 1].data.clone();
            self.data.remove(self.cursor.y - 1);
            self.cursor_up();
            self.data[self.cursor.y - 1]
                .data
                .extend_from_slice(data.as_slice());
            self.cursor.x = self.data[self.cursor.y - 1].len() - data.len() + 1;
        } else {
            self.cursor.left();
            let row = &mut self.data[self.cursor.y - 1];
            row.data.remove(self.cursor.x - 1);
        }
        self.mark_dirty();
    }

    // Enter button, not 'o'
    pub fn new_line(&mut self) {
        let current_row = &self.data[self.cursor.y - 1];

        if self.cursor.x - 1 != current_row.data.len() {
            let row = self.data.remove(self.cursor.y - 1);
            let data = row.data.leak();
            let x = self.cursor.x - 1;
            self.data
                .insert(self.cursor.y - 1, ERow::new(data[..x].to_vec()));
            self.data
                .insert(self.cursor.y, ERow::new(data[x..].to_vec()));
        } else {
            self.data.insert(self.cursor.y, ERow::empty(0));
        }

        self.cursor.down();
        self.cursor.move_to_line_beginning();
        self.mark_dirty();
    }
}
