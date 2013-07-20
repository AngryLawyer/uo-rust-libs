pub struct Buffer<T> {
    items: ~[T],
    length: uint,
    pos: uint
}

impl<T: Clone> Buffer<T> {

    pub fn new(items: ~[T]) -> Buffer<T> {
        let len = items.len();

        Buffer {
            items: items,
            length: len,
            pos: 0
        }
    }

    fn eof(&self) -> bool {self.pos == self.length}

    pub fn read_items(&mut self, number: uint) -> ~[T] {
        assert!(number + self.pos <= self.length);
        let return_data = self.items.slice(self.pos, self.pos + number);
        self.pos += number;
        return return_data.to_owned();
    }

    pub fn seek(&mut self, pos: uint) {
        assert!(pos <= self.length);
        self.pos = pos;
    }
}

pub fn bytes_to_le_uint(bytes: ~[u8]) -> uint {

    let mut val = 0u;
    let mut pos = 0u;
    let mut i = 0;

    let size = bytes.len();

    while i < size {
        val += bytes[i] as uint << pos;
        pos += 8u;
        i += 1;
    }

    return val;
}

pub fn bytes_to_be_uint(bytes: ~[u8]) -> uint {
    let mut val = 0u;
    let mut i = bytes.len(); 

    while i > 0u {
        i -= 1u;
        val += (bytes[i] as uint) << i * 8u;
    }
    return val;
}

pub fn uint_to_le_bytes(n: u64, size: uint) -> ~[u8] {
    match size {
        1u => { return ~[n as u8] }
        2u => { return ~[n as u8, (n >> 8) as u8] }
        4u => { return ~[n as u8, (n >> 8) as u8, (n >> 16) as u8, (n >> 24) as u8] }
        8u => { return ~[n as u8,
            (n >> 8) as u8,
            (n >> 16) as u8,
            (n >> 24) as u8,
            (n >> 32) as u8,
            (n >> 40) as u8,
            (n >> 48) as u8,
            (n >> 56) as u8] 
        }
        _ => {
            let mut bytes: ~[u8] = ~[];
            let mut i = size;
            let mut n = n;

            while i > 0u {
                bytes.push((n & 255_u64) as u8);
                n >>= 8_u64;
                i -= 1u;
            }
            return bytes;
        }
    }
}

pub fn u8vec_to_u16vec(input: ~[u8]) -> ~[u16] {
    let mut output: ~[u16] = ~[];
    let len = input.len();
    let mut i:uint = 0;
    
    assert!(len % 2 == 0);

    while (i < (len / 2)) {
        output.push((input[i * 2] as u16 + ((input[(i * 2) + 1] as u16) << 8)));
        i += 1;
    }

    return output;
}
