use super::cursor::ECursor;
use std::{fs::File, io::Read};

pub struct ERow {
    data: Vec<u8>,
}

impl ERow {
    fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
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

    pub(crate) fn cursor_up(&mut self) {
        self.cursor.up();
    }

    pub(crate) fn cursor_down(&mut self) {
        self.cursor.down();
    }

    pub(crate) fn cursor_left(&mut self)  {
        self.cursor.left();
    }

    pub(crate) fn cursor_right(&mut self) {
        self.cursor.right();
    }

    pub(crate) fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub(crate) fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub(crate) fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub(crate) fn insert(&mut self, data: &[u8])  {
        let row = &mut self.rows[self.cursor.y-1];
        let mut new_row = vec![]; 
        new_row.extend_from_slice(&row.data[..self.cursor.x-1]);
        new_row.extend_from_slice(data);
        new_row.extend_from_slice(&row.data[self.cursor.x-1..]);
        row.data = new_row.to_vec(); 
        self.cursor.right();
        self.mark_dirty();
    }

    pub(crate) fn backspace(&mut self) {
        self.cursor.left();
        let row = &mut self.rows[self.cursor.y-1];
        let mut new_row = vec![]; 
        new_row.extend_from_slice(&row.data[..self.cursor.x-1]);
        new_row.extend_from_slice(&row.data[self.cursor.x..]);
        row.data = new_row.to_vec(); 
        self.mark_dirty();
    }
}
