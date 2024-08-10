use std::io::{Result, Write};

pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }

    // `Write` has a `write_all` method we don't implement
    // since the trait already provides a default implementation!
}

fn main() {}
