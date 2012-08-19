export bytes_to_le_uint;
export bytes_to_be_uint;
export uint_to_le_bytes;
export u8vec_to_u16vec;
export read_le_u16;
export read_le_u32;

fn bytes_to_le_uint(bytes: ~[u8]) -> uint {

    let mut val = 0u, pos = 0u, i = 0;
    let size:uint = vec::len(bytes);

    while i < size {
        val += bytes[i] as uint << pos;
        pos += 8u;
        i += 1;
    }

    return val;
}

fn bytes_to_be_uint(bytes: ~[u8]) -> uint {
    let mut val = 0u, i = vec::len(bytes);
    while i > 0u {
        i -= 1u;
        val += (bytes[i] as uint) << i * 8u;
    }
    return val;
}

fn uint_to_le_bytes(n: u64, size: uint) -> ~[u8] {
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
            let mut bytes: ~[u8] = ~[], i = size, n = n;
            while i > 0u {
                vec::push(bytes, (n & 255_u64) as u8);
                n >>= 8_u64;
                i -= 1u;
            }
            return bytes;
        }
    }
}

fn u8vec_to_u16vec(input: ~[u8]) -> ~[u16] {
    let mut output: ~[u16] = ~[];
    let len = vec::len(input);
    let mut i:uint = 0;
    
    assert len % 2 == 0;

    while (i < (len / 2)) {
        vec::push(output, (input[i * 2] as u16 + ((input[(i * 2) + 1] as u16) << 8)));
        i += 1;
    }

    return output;
}

fn read_le_u16(&input: ~[u8]) -> u16 {
    let first  = vec::shift(input);
    let second = vec::shift(input);
    ret bytes_to_le_uint(~[first, second]) as u16;
//    ret first | (second << 8);
}

fn read_le_u32(&input: ~[u8]) -> u32 {
    let first = vec::shift(input);
    let second = vec::shift(input);
    let third = vec::shift(input);
    let fourth = vec::shift(input);
    ret bytes_to_le_uint(~[first, second, third, fourth]) as u32;
    //ret first | (second << 8) | (third << 16) | (fourth << 24);
}
