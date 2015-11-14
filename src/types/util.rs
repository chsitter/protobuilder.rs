use std::io;
use std::io::{Read, Write};

pub fn decode_u16(src: &mut Read) -> io::Result<u16> {
    let mut buf = [0u8; 2];
    assert_eq!(src.read(&mut buf).is_ok(), true);
    Ok(((buf[0] as u16) << 8 ) | (buf[1] as u16))
}

pub fn encode_u16(value: &u16, dst: &mut Write) -> io::Result<usize> {
    let mut buf = [0u8; 2];
    buf[0] = (value >> 8 & 0xffu8 as u16) as u8;
    buf[1] = (value & 0xffu8 as u16) as u8;

    assert_eq!(dst.write(&buf).is_ok(), true);
    Ok(2)
}

pub fn decode_i32(src: &mut Read) -> io::Result<i32> {
    let mut buf = [0u8; 4];
    assert_eq!(src.read(&mut buf).is_ok(), true);

    Ok(((buf[0] as i32) << 24 ) | (buf[1] as i32) << 16 | (buf[2] as i32)  << 8 | (buf[3] as i32))
}

pub fn encode_i32(value: &i32, dst: &mut Write) -> io::Result<usize> {
    let mut buf = [0u8; 4];
    buf[0] = (value >> 24 & 0xffu8 as i32) as u8;
    buf[1] = (value >> 16 & 0xffu8 as i32) as u8;
    buf[2] = (value >> 8 & 0xffu8 as i32) as u8;
    buf[3] = (value & 0xffu8 as i32) as u8;

    assert_eq!(dst.write(&buf).is_ok(), true);
    Ok(4)
}

pub fn decode_usize(src: &mut Read) -> io::Result<usize> {
    let mut buf = [0u8; 2];
    assert_eq!(src.read(&mut buf).is_ok(), true);
    Ok(((buf[0] as usize) << 8 ) | (buf[1] as usize))
}

pub fn encode_usize(value: &usize, dst: &mut Write) -> io::Result<usize> {
    let mut buf = [0u8; 2];
    buf[0] = (value >> 8 & 0xffu8 as usize) as u8;
    buf[1] = (value & 0xffu8 as usize) as u8;

    assert_eq!(dst.write(&buf).is_ok(), true);
    Ok(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_u16() {
        let buf = vec![1u8,0u8];

        assert_eq!(256, decode_u16(&mut &buf[..]).unwrap());
    }

    #[test]
    fn test_encode_u16() {
        let mut vec: Vec<u8> = Vec::new();
        encode_u16(&256, &mut vec);
        
        assert_eq!(vec![0x01, 0x00], vec);
    }

    #[test]
    fn test_encode_u16_1() {
        let mut vec: Vec<u8> = Vec::new();
        encode_u16(&1, &mut vec);

        assert_eq!(vec![0x00, 0x01], vec);
    }
}
