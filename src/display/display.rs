use crate::{
    editor::{cursor::ECursor, session::ERow},
    writer::escapes::EscapeSequence,
};
use std::io::{Stdout, Write};

// Viewport tell's use what part of the buffer we are currently viewing
#[derive(Debug)]
pub struct Viewport {
    // Max width of the viewport
    width: u16,

    // Max height of the viewport
    height: u16,

    // How many characters we have scrolled to the right
    pub offset_x: u16,

    // How many characters we have scrolled to the bottom
    pub offset_y: u16,
}

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Viewport {
    pub fn with_dimensions(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            offset_x: 0,
            offset_y: 0,
        }
    }

    pub fn offset_y(&self) -> usize {
        self.offset_y as usize
    }

    pub fn offset_x(&self) -> usize {
        self.offset_x as usize
    }
}

pub struct Cells {
    pub x: Vec<usize>,
    pub y: Vec<usize>,
    pub chars: Vec<char>,
}

pub struct DisplayOptions {
    show_line_numbers: bool,
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
        for i in 0..self.x.len() {
            // Move it out of the loop and zero after each iteration?
            let mut utf8_buffer = [0u8; 4];
            EscapeSequence::MoveCursor(self.x[i], self.y[i])
                .execute(writer)
                .unwrap();
            self.chars[i].encode_utf8(&mut utf8_buffer);
            writer.write(&utf8_buffer).unwrap();
        }
    }
}

pub struct Display {
    pub viewport: Viewport,
    cells: Cells,
}

impl Display {
    pub fn with_dimensions(width: u16, height: u16) -> Self {
        Self {
            viewport: Viewport::with_dimensions(width, height),
            cells: Cells::new(width as usize * height as usize),
        }
    }

    pub fn height(&self) -> u16 {
        self.viewport.height
    }

    // Refresh the display buffer
    pub fn refresh(&mut self, data: &[ERow], display_options: DisplayOptions) {
        self.cells = Cells::new(self.viewport.height as usize * self.viewport.width as usize);

        let offset_x = if display_options.show_line_numbers {
            // TODO: Calculate this from total lines
            4
        } else {
            0
        };

        let mut idx = 0;
        let max_lines = data.len().min(self.viewport.height as usize);
        let start_line = self.viewport.offset_y as usize;

        let printable_lines = data.len().min(start_line + max_lines);
        for row in start_line..printable_lines {
            let rd = data[row].data.value();
            let display_row = row - start_line;

            if display_options.show_line_numbers {
                let row_number = (row + 1).to_string();
                let mut chars = row_number.chars();

                let whitespaces = offset_x - row_number.len();
                for i in 1..whitespaces {
                    self.cells.x[idx] = i;
                    self.cells.y[idx] = display_row + 1;
                    self.cells.chars[idx] = ' ';
                    idx += 1
                }

                for i in whitespaces..=offset_x - 1 {
                    self.cells.x[idx] = i;
                    self.cells.y[idx] = display_row + 1;
                    self.cells.chars[idx] = chars.next().unwrap_or(' ');
                    idx += 1;
                }
            }

            // TODO: Skip some chars if they are out of viewport
            let start_col = self.viewport.offset_x as usize;
            for (col, c) in rd
                .chars()
                .skip(start_col)
                .take(self.viewport.width as usize - offset_x)
                .enumerate()
            {
                self.cells.x[idx] = offset_x + col + 1;
                self.cells.y[idx] = display_row + 1;
                self.cells.chars[idx] = c;
                idx += 1;
            }
        }
    }

    pub(crate) fn width(&self) -> usize {
        self.viewport.width as usize
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn point_at(&self, cursor: &ECursor) -> Point {
        let x = cursor.x_relative_to_viewport(&self.viewport);
        let y = cursor.y_relative_to_viewport(&self.viewport);
        Point { x, y }
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
