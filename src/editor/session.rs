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
    file: Option<File>, // TODO: This may be optional tho!!!
    rows: Vec<ERow>,
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
            file: Some(file),
            rows,
            cursor: ECursor::at_home(),
            dirty: true,
        };
        Ok(session)
    }

    pub fn rows(&self) -> &[ERow] {
        self.rows.as_ref()
    }

    pub fn cursor(&self) -> &ECursor {
        &self.cursor
    }

    pub fn cursor_up(&mut self) {
        self.cursor.up();
        if self.cursor.x > self.rows[self.cursor.y - 1].len() {
            self.cursor.x = self.rows[self.cursor.y - 1].len();
        }
    }

    pub fn cursor_down(&mut self) {
        if self.cursor.y != self.rows.len() {
            self.cursor.down();
        }
        if self.cursor.x > self.rows[self.cursor.y - 1].len() {
            self.cursor.x = self.rows[self.cursor.y - 1].len();
        }
    }

    pub fn cursor_left(&mut self) {
        self.mark_dirty();
        self.cursor.left();
    }

    pub fn cursor_right(&mut self) {
        if self.cursor.x < self.rows[self.cursor.y - 1].len() {
            self.cursor.right();
        }
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
        let row = &mut self.rows[self.cursor.y - 1];

        for bytes in data {
            row.data.insert(self.cursor.x - 1, *bytes);
            self.cursor.right();
        }

        self.mark_dirty();
    }

    // TODO: Refactor maybe to use some commands?
    pub fn backspace(&mut self) {
        if self.cursor.x <= 1 {
            return;
        }
        self.cursor.left();
        let row = &mut self.rows[self.cursor.y - 1];
        row.data.remove(self.cursor.x - 1);
        self.mark_dirty();
    }

    pub fn new_line(&mut self) {
        self.rows.insert(self.cursor.y, ERow::empty(0));
        self.cursor_down();
        self.cursor.x = 1;
        self.mark_dirty();
    }

    pub fn total_bytes(&self) -> usize {
        self.rows.iter().map(|it| it.data.capacity()).sum()
    }
}
