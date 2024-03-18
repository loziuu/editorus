use crate::{editor::session::ERow, writer::escapes::EscapeSequence};
use std::{
    io::{BufWriter, Stdout, Write},
    time::Instant,
};

struct Dimensions(u16, u16);

pub struct Cells {
    pub x: Vec<usize>,
    pub y: Vec<usize>,
    pub chars: Vec<char>,
}

impl Cells {
    pub fn new(count: usize) -> Self {
        Cells {
            x: vec![0; count],
            y: vec![0; count],
            chars: vec!['\0'; count],
        }
    }

    pub(crate) fn write_to(&self, writer: &mut BufWriter<&mut Stdout>) {
        for i in 0..self.x.len() {
            EscapeSequence::MoveCursor(self.x[i], self.y[i]).execute_buffered(writer);
            writer.write(&[self.chars[i] as u8; 1]).unwrap();
        }
    }
}

pub struct Display {
    dimensions: Dimensions,
    // TODO: This shit should be sorted?
    pub cells: Cells,
    // changes: Vec<Cell>, <- update only changes.
}

impl Display {
    pub fn with_dimensions(width: u16, height: u16) -> Self {
        Self {
            dimensions: Dimensions(width, height),
            cells: Cells::new(width as usize * height as usize),
        }
    }

    pub fn height(&self) -> u16 {
        self.dimensions.1
    }

    pub fn refresh(&mut self, data: &[ERow]) {
        let mut idx = 0;
        for row in 0..data.len() {
            let rd = data[row].data.value();
            let row_data = rd.as_bytes();

            for col in 0..row_data.len() {
                self.cells.x[idx] = col + 1;
                self.cells.y[idx] = row + 1;
                self.cells.chars[idx] = char::from_u32(row_data[col] as u32).unwrap();
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
        let started = Instant::now();
        let mut writer = BufWriter::with_capacity(65535, sink);
        self.display.cells.write_to(&mut writer);
        writer.flush().unwrap();
        println!("Dump_to took {:?}.", started.elapsed());
    }
}
