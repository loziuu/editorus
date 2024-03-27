use super::{config::Configuration, cursor::ECursor};
use crate::{
    display::display::{Display, Dump, WholeDump},
    rope::rope::Rope,
};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Read, Stdout, Write},
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
    data: Vec<ERow>,
    display: Display,
    cursor: ECursor,
    dirty: bool,
    fd: Option<String>,
}

impl Session {
    pub fn new(width: u16, height: u16) -> Self {
        let session = Session {
            data: vec![ERow::empty()], // It's time to move to rope :)
            display: Display::with_dimensions(width, height),
            cursor: ECursor::at_home(),
            dirty: true,
            fd: None,
        };
        session
    }

    pub fn with_config(width: u16, height: u16, config: Configuration) -> Self {
        // Offset_X should be calculated based on line numbers
        let cursor_offset_x = if config.show_line_numbers { 4 } else { 0 };

        let session = Session {
            data: vec![ERow::empty()],
            display: Display::with_dimensions(width, height),
            cursor: ECursor::with_offset(cursor_offset_x, 0),
            dirty: true,
            fd: None,
        };
        session
    }

    pub fn open_file(&mut self, file_path: String) -> Result<(), std::io::Error> {
        let mut content = String::new();
        let mut file = OpenOptions::new().read(true).open(&file_path)?;
        file.read_to_string(&mut content)?;
        let rows: Vec<ERow> = content
            .lines()
            .into_iter()
            .map(|row| ERow::from(row))
            .collect();
        if rows.is_empty() {
            self.data.push(ERow::empty());
        } else {
            self.data = rows;
        }
        self.fd = Some(file_path);
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
        if self.cursor.y == 1 && self.cursor.offset.1 > 0 && self.display.viewport.offset_y() > 0 {
            self.cursor.offset.1 -= 1;
            self.display.viewport.offset_y -= 1;
            self.mark_dirty();
        } else {
            self.cursor.up();

            if self.cursor.x > self.data[self.cursor.y - 1].len() {
                self.cursor.x = self.data[self.cursor.y - 1].len();
            }

            if self.cursor.x == 0 {
                self.cursor.x = 1;
            }
        }
    }

    pub fn cursor_down(&mut self) {
        // Change viewport if possible
        if self.cursor.y + self.display.viewport.offset_y() != self.data.len() {
            if self.cursor.y == self.display.height() as usize {
                // CAN WE GROUP IT SOMEHOW?
                self.display.viewport.offset_y += 1;
                self.cursor.offset.1 += 1;
                self.mark_dirty();
            } else {
                self.cursor.down();
                if self.cursor.x > self.data[self.cursor.y - 1].len() {
                    self.cursor.x = self.data[self.cursor.y - 1].len() + 1;
                }
            }
        }
        log::info!("{:?}", self.cursor);
    }

    pub fn cursor_left(&mut self) {
        // Change viewport if possible
        if self.cursor.x == 1 {
            if self.cursor.y != 1 {
                self.cursor.up();
                self.cursor.x = self.data[self.cursor.y - 1].len() + 2; // (cursor.x = 1) == data[0]
            }
        }
        self.cursor.left();
    }

    pub fn cursor_right(&mut self) {
        if self.cursor.x <= self.data[self.cursor.y - 1].len() {
            self.cursor.right();
        } else {
            // Refactor so cursor_down() return result.
            let curr_row = self.cursor.y;
            self.cursor_down();
            if self.cursor.y != curr_row {
                self.cursor.move_to_line_beginning();
            }
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
        self.rebuild_display();
    }

    fn rebuild_display(&mut self) {
        self.display.refresh(&self.data, Default::default());
    }

    pub fn insert(&mut self, data: &[u8]) {
        let row = &mut self.data[self.cursor.y_relative()];
        let data = std::str::from_utf8(data).unwrap();
        row.data.insert(self.cursor.x_relative(), data);
        self.cursor.right();
        self.mark_dirty();
    }

    pub fn backspace(&mut self) {
        if self.cursor.x == 1 && self.cursor.y == 1 {
            return;
        }
        if self.cursor.at_start() {
            let prev_row = self.data.remove(self.cursor.y_relative() - 1);
            let curr_row = self.data.remove(self.cursor.y_relative() - 1); 
            let x_final_position = prev_row.len();
            let concat = prev_row.data.concat(curr_row.data);
            self.data.insert(self.cursor.y_relative() - 1, ERow::new(concat));
            self.cursor.up();
            self.cursor.x = x_final_position + 1;
        } else {
            self.cursor.left();
            let row = &mut self.data[self.cursor.y_relative()];
            row.data.remove_at(self.cursor.x_relative());
        }
        self.mark_dirty();
    }

    pub fn new_line(&mut self) {
        let current_row = &self.data[self.cursor.y - 1];
        if self.cursor.x_relative() != current_row.data.len() {
            let row = &mut self.data[self.cursor.y - 1];
            let (curr, next) = row.data.split_at(self.cursor.x_relative());
            row.data = curr;
            self.data.insert(self.cursor.y, ERow::new(next));
        } else {
            self.data.insert(self.cursor.y, ERow::empty());
        }

        self.cursor.down();
        self.cursor.move_to_line_beginning();

        self.mark_dirty();
    }

    pub fn delete(&mut self) {
        if self.is_cursor_at_the_end_of_line() {
            if self.cursor.y == self.data.len() {
                return;
            }
            let curr_line = self.data.remove(self.cursor.y_relative());
            let next_line = self.data.remove(self.cursor().y_relative());

            let new_line = curr_line.data.concat(next_line.data);
            self.data
                .insert(self.cursor.y_relative(), ERow::new(new_line));
        } else {
            let row = &mut self.data[self.cursor.y_relative()];
            row.data.remove_at(self.cursor.x_relative());
        }
        self.mark_dirty();
    }

    fn is_cursor_at_the_end_of_line(&self) -> bool {
        self.cursor.x_relative() == self.data[self.cursor.y - 1].data.len()
    }

    pub(crate) fn display_on(&mut self, stdout: &mut Stdout) -> std::io::Result<()> {
        WholeDump::new(&self.display).dump_to(stdout);
        self.mark_clean();
        Ok(())
    }

    // Handle for empty buffer once we have it
    pub fn save_file(&self) {
        if let Some(file_path) = &self.fd {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_path)
                .unwrap();
            let mut file_writer = BufWriter::new(file);
            for row in &self.data {
                file_writer.write_all(row.data.value().as_bytes()).unwrap();
                // Get line ending per system
                file_writer.write_all(b"\n").unwrap();
            }
            file_writer.flush().unwrap();
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weird_stuff() {
        let config = crate::editor::config::Configuration {
            show_line_numbers: true,
        };
        let mut session = Session::with_config(50, 50, config);

        session.insert(b"a");
        session.insert(b"s");
        session.insert(b"d");
        session.insert(b"f");
        session.insert("ś".as_bytes());

        session.new_line();
        session.insert(b"q");
        session.insert(b"w");
        session.insert(b"e");
        session.insert(b"r");

        session.cursor_left();
        session.cursor_left();
        session.cursor_left();
        session.cursor_left();
        session.backspace();

        println!("{:?}", session.cursor());

        session.new_line();

        assert_eq!(session.data[0].data.value(), "asdfś");
        assert_eq!(session.data[1].data.value(), "qwer");
    }

    #[test]
    fn load_file() {
        let mut session = Session::new(50, 50);
        session.open_file("witam.txt".to_string()).unwrap();

        assert_eq!(session.data[0].data.value(), "Witam");
    }

    #[test]
    fn load_file_add_letters_delete() {
        let config = crate::editor::config::Configuration {
            show_line_numbers: true,
        };
        let mut session = Session::with_config(50, 50, config);
        session.open_file("witam.txt".to_string()).unwrap();

        assert_eq!(session.data[0].data.value(), "Witam");

        session.insert(b"N");
        session.insert(b"c");
        session.insert(b"c");

        session.cursor_left();
        session.cursor_left();
        session.cursor_left();
        session.delete();
        session.delete();
        session.delete();

        assert_eq!(session.data[0].data.value(), "Witam");
    }
}
