use std::io;
use std::io::prelude::*;
use super::Endec;
use std::iter::FromIterator;

impl<X:Endec> Endec for Vec<X> where X: Endec {

    fn encode(value: &Vec<X>, dst: &mut Write) -> io::Result<usize> {
        let arrlen = value.len();
        let mut len = try!(usize::encode(&arrlen, dst));
        for elem in value {
            len += try!(<X as Endec>::encode(elem, dst));
        }
        Ok(len)
    }

    fn decode(src: &mut Read) -> io::Result<Vec<X>> {
        let len:usize = usize::decode(src).unwrap();
        Ok(Vec::from_iter((0..len).map(|_| <X as Endec>::decode(src).unwrap())))
    }
}


#[cfg(test)]
mod tests {
    use Endec;

    #[test]
    fn test_decode_string_array() {
        let buf = vec![0x00, 0x01, 0x00, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
        let arr = <Vec<String> as Endec>::decode(&mut &buf[..]).unwrap();

        assert_eq!(1, arr.len());
        assert_eq!("hello".to_string(), arr[0]);
    }

    #[test]
    fn test_decode_u16_array() {
        let buf = vec![0x00, 0x02, 0x00, 0x01, 0x00, 0x02];
        let arr = <Vec<u16> as Endec>::decode(&mut &buf[..]).unwrap();

        assert_eq!(2, arr.len());
        assert_eq!(vec![1,2], arr);
    }

    #[test]
    fn test_encode_string_array() {
        let expected = vec![0x00, 0x01, 0x00, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
        let mut vec: Vec<u8> = Vec::new();

        let encoded_len = <Vec<String> as Endec>::encode(&vec!["hello".to_string()], &mut vec).unwrap();

        assert_eq!(9, encoded_len);
        assert_eq!(expected, vec);
    }

    #[test]
    fn test_encode_u16_array() {
        let expected = vec![0x00, 0x02, 0x00, 0x01, 0x00, 0x02];
        let mut vec = Vec::new();

        let encoded_len = <Vec<u16> as Endec>::encode(&vec![1,2], &mut vec).unwrap();

        assert_eq!(6, encoded_len);
        assert_eq!(expected, vec);
    }
}
