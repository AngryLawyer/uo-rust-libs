export bytes_to_le_uint;
export uint_to_le_bytes;

fn bytes_to_le_uint(bytes: ~[u8]) -> uint {

    let mut val = 0u, pos = 0u, i = 0;
    let size:uint = vec::len(bytes);

    while i < size {
        val += bytes[i] as uint << pos;
        pos += 8u;
        i += 1
    }

    ret val;
}

fn uint_to_le_bytes(n: u64, size: uint) -> ~[u8] {
    alt size {
        1u { ret ~[n as u8] }
        2u { ret ~[n as u8, (n >> 8) as u8] }
        4u { ret ~[n as u8, (n >> 8) as u8, (n >> 16) as u8, (n >> 24) as u8] }
        8u { ret ~[n as u8,
            (n >> 8) as u8,
            (n >> 16) as u8,
            (n >> 24) as u8,
            (n >> 32) as u8,
            (n >> 40) as u8,
            (n >> 48) as u8,
            (n >> 56) as u8] 
        }
        _ {
            let mut bytes: ~[u8] = ~[], i = size, n = n;
            while i > 0u {
                vec::push(bytes, (n & 255_u64) as u8);
                n >>= 8_u64;
                i -= 1u;
            }
            ret bytes;
        }
    }
}
