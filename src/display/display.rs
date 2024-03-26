use crate::{editor::session::ERow, writer::escapes::EscapeSequence};
use std::io::{Stdout, Write};

struct Viewport(u16, u16);

pub struct Cells {
    pub x: Vec<usize>,
    pub y: Vec<usize>,
    pub chars: Vec<char>,
}

pub struct DisplayOptions {
    show_line_numbers: bool,
    // view_offset?
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self {
            show_line_numbers: true,
        }
    }
}

impl Cells {
    pub fn new(count: usize) -> Self {
        Cells {
            x: vec![0; count],
            y: vec![0; count],
            chars: vec!['\0'; count],
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Stdout) {
        // TODO: This might be moved into loop.
        let mut utf8_buffer = [0u8; 4];
        for i in 0..self.x.len() {
            EscapeSequence::MoveCursor(self.x[i], self.y[i]).execute(writer)
                .unwrap();
            self.chars[i].encode_utf8(&mut utf8_buffer);
            writer.write(&utf8_buffer).unwrap();
        }
    }
}

// Add viewport
pub struct Display {
    viewport: Viewport,
    cells: Cells,
}

impl Display {
    pub fn with_dimensions(width: u16, height: u16) -> Self {
        Self {
            viewport: Viewport(width, height),
            cells: Cells::new(width as usize * height as usize),
        }
    }

    pub fn height(&self) -> u16 {
        self.viewport.1
    }

    // Refresh the display buffer
    pub fn refresh(&mut self, data: &[ERow], display_options: DisplayOptions) {
        self.cells = Cells::new(self.viewport.0 as usize * self.viewport.1 as usize);

        let offset_x = if display_options.show_line_numbers {
            // TODO: Calculate this from total lines
            4
        } else {
            0
        };

        let mut idx = 0;
        // TODO: Skip some lines if they are out of viewport
        for row in 0..data.len() {
            let rd = data[row].data.value();

            if display_options.show_line_numbers {
                let row_number = (row + 1).to_string();
                let mut chars = row_number.chars();

                let whitespaces = offset_x - row_number.len();
                for i in 1..whitespaces {
                    self.cells.x[idx] = i;
                    self.cells.y[idx] = row + 1;
                    self.cells.chars[idx] = ' ';
                    idx += 1
                }

                for i in whitespaces..=offset_x-1 {
                    self.cells.x[idx] = i;
                    self.cells.y[idx] = row + 1;
                    self.cells.chars[idx] = chars.next().unwrap_or(' ');
                    idx += 1;
                }
            }

            // TODO: Skip some chars if they are out of viewport
            for (col, c) in rd.chars().enumerate() {
                self.cells.x[idx] = offset_x + col + 1;
                self.cells.y[idx] = row + 1;
                self.cells.chars[idx] = c;
                idx += 1;
            }
        }
    }
}

pub struct WholeDump<'a> {
    display: &'a Display,
}

impl<'a> WholeDump<'a> {
    pub fn new(display: &'a Display) -> Self {
        Self { display }
    }
}

pub trait Dump {
    fn dump_to(&self, sink: &mut Stdout);
}

impl<'a> Dump for WholeDump<'a> {
    fn dump_to(&self, sink: &mut Stdout) {
        self.display.cells.write_to(sink);
        sink.flush().unwrap();
    }
}
