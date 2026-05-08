use std::io::{Cursor, Read, Seek, ExactSizeIterator, Result};
pub struct ExactSizeCursor<T>(Cursor<Vec<T>>, usize);

impl<T> ExactSizeCursor<T> {
    pub fn new(vec: Vec<T>) -> Self {
        let len = vec.len();
        ExactSizeCursor(Cursor::new(vec), len)
    }
}

impl<T> Read for ExactSizeCursor<T> {
    fn read(&mut self, buf: &mut[u8]) -> Result<usize> {
        self.0.read(buf)
    }
}
