use std::io::Stdout;

pub enum DisplayBufferSink {
    Stdout(Stdout),    
}

impl std::io::Write for DisplayBufferSink {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            DisplayBufferSink::Stdout(stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            DisplayBufferSink::Stdout(stdout) => stdout.flush(),
        }
    }
}
