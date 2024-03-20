// Add offset
struct Offset(usize, usize);

pub struct ECursor {
    pub x: usize,
    pub y: usize,
    offset: Offset,
}

impl ECursor {
    pub fn at_home() -> ECursor {
        Self {
            x: 1,
            y: 1,
            offset: Offset(0, 0),
        }
    }

    pub fn with_offset(x: usize, y: usize) -> Self {
        Self {
            x: 1,
            y: 1,
            offset: Offset(x, y),
        }
    }

    pub(crate) fn right(&mut self) {
        self.x += 1;
    }

    pub(crate) fn left(&mut self) {
        if self.x > 1 {
            self.x -= 1;
        }
    }

    fn min_x(&self) -> usize {
        1 + self.offset.0
    }

    pub(crate) fn down(&mut self) {
        self.y += 1;
    }

    pub(crate) fn up(&mut self) {
        if self.y != self.offset.1 + 1 {
            self.y -= 1;
        }
    }

    pub(crate) fn at_start(&self) -> bool {
        self.x == 1 && self.y != 1 
    }

    pub(crate) fn move_to_line_beginning(&mut self) {
        self.x = 1
    }

    pub(crate) fn x(&self) -> usize {
        self.get_x()
    }

    pub(crate) fn x_relative(&self) -> usize {
        self.x - 1
    }

    fn get_x(&self) -> usize {
        self.x + self.offset.0
    }
}
