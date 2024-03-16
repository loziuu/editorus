use super::sink::DisplayBufferSink;
use crate::{editor::session::ERow, writer::escapes::EscapeSequence};
use std::io::{Stdout, Write};

struct Dimensions(u16, u16);

struct Cell {
    x: usize,
    y: usize,
    data: char, // char?
}

impl Cell {}

pub struct Display {
    dimensions: Dimensions,
    // TODO: This shit should be sorted?
    cells: Vec<Cell>,
    // changes: Vec<Cell>, <- update only changes.
}

impl Display {
    pub fn with_dimensions(width: u16, height: u16) -> Self {
        Self {
            dimensions: Dimensions(width, height),
            cells: Vec::with_capacity(width as usize * height as usize),
        }
    }

    pub fn height(&self) -> u16 {
        self.dimensions.1
    }

    pub fn display_all(&mut self, data: &[ERow]) {
        let mut cells = Vec::with_capacity(self.dimensions.0 as usize * self.dimensions.1 as usize);

        for row in 0..data.len() {
            let row_data = data[row].data();

            for col in 0..row_data.len() {
                cells.push(Cell {
                    x: col + 1,
                    y: row + 1,
                    data: char::from_u32(row_data[col] as u32).unwrap(),
                })
            }
        }

        self.cells = cells;
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
    // This is very sub optimal
    fn dump_to(&self, sink: &mut Stdout) {
        for cell in self.display.cells.iter() {
            EscapeSequence::from(cell)
                .execute(sink)
                .expect("Failed to execute escape sequence.");
            sink.write(&[cell.data as u8; 1])
                .expect("Failed to write character.");
        }
    }
}

// TODO: Check if similar trait exisits in Rust std lib
trait Baiter<'a> {
    // Maybe get whole display?
    fn to_bytes(cell: &'a Cell) -> &'a [u8];
}

impl From<&Cell> for EscapeSequence {
    fn from(cell: &Cell) -> Self {
        EscapeSequence::MoveCursor(cell.x as usize, cell.y as usize)
    }
}
