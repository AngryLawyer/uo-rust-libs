pub const MEMWRITER_ERROR:&'static str = "MemWriter unexpectedly failed";

pub struct DataBuffer<'r, T: 'static> {
    pos: uint,
    buffer: &'r[T]
}

impl <'r, T> DataBuffer<'r, T> {

    pub fn new(buffer: &'r[T]) -> DataBuffer<'r, T> {
        DataBuffer {
            pos: 0,
            buffer: buffer
        }
    }

    pub fn read(&mut self) -> &T {
        let item = &self.buffer[self.pos];
        self.pos += 1;
        item
    }
}
