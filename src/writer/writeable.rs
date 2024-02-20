use std::io::{Stdout, Write};

pub(super) trait Writeable {
    fn write_to(&mut self, val: &[u8]);
    fn flush_now(&mut self);
}

impl Writeable for Stdout {
    fn write_to(&mut self, val: &[u8]) {
        // TODO: Add some buffer maybe?
        self.write(val).unwrap();
    }

    fn flush_now(&mut self) {
        self.flush().unwrap();
    }
}
