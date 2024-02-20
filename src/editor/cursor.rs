pub struct ECursor {
    pub x: usize,
    pub y: usize,
}

impl ECursor {
    pub fn at_home() -> ECursor {
        Self { x: 1, y: 1 }
    }

    pub(crate) fn right(&mut self) {
        self.x += 1;
    }

    pub(crate) fn left(&mut self) {
        if self.x != 1 {
            self.x -= 1;
        }
    }

    pub(crate) fn down(&mut self) {
        self.y += 1;
    }

    pub(crate) fn up(&mut self) {
        if self.y != 1 {
            self.y -= 1;
        }
    }
}
