use std::io;
use std::io::prelude::*;
use std::str;
use super::Endec;

impl Endec for String {

    fn encode(value: &String, dst: &mut Write) -> io::Result<usize> {
        let mut buf: Vec<u8> = Vec::with_capacity(value.len());

        let mut len = try!(<u16 as Endec>::encode(&(value.len() as u16), dst));
        for c in value.chars() {
            buf.push(c as u8)
        }
        len += try!(dst.write(&buf));
        Ok(len)
    }

    fn decode(src: &mut Read) -> io::Result<String> {
        let strlen = <u16 as Endec>::decode(src).unwrap();
        let mut buf: Vec<u8> = Vec::with_capacity(strlen as usize);

        try!(src.take(strlen as u64).read_to_end(&mut buf));

        let string = str::from_utf8(&buf).unwrap();
        Ok(string.to_string())
    }
}

#[cfg(test)]
mod tests {
    use Endec;

    #[test]
    fn test_decode_string() {
        let buf = vec![0u8, 3u8, 65u8, 66u8, 67u8];
        let string = String::decode(&mut &buf[..]).unwrap();
    
        assert_eq!(3, string.len());
        assert_eq!("ABC", string);
    }

    #[test]
    fn test_encode_string() {
        let mut vec: Vec<u8> = Vec::new();
        let expected = vec![0u8, 3u8, 65u8, 66u8, 67u8];
        let len = String::encode(&"ABC".to_string(), &mut vec).unwrap();
   
        assert_eq!(5, len);
        assert_eq!(expected, vec);
    }
}
