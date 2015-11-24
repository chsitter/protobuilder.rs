use std::io;
use std::io::prelude::*;
use super::Endec;
use std::marker::PhantomData;
use std::iter::FromIterator;
use num::{NumCast, ToPrimitive};


#[derive(PartialEq, Debug)]
pub struct Arr<X:Endec, Y> (PhantomData<X>, Vec<Y>);

impl <X:Endec, Y:Endec> Endec for Arr<X, Y> where X:NumCast {
    fn encode(value: &Arr<X, Y>, dst:&mut Write) -> io::Result<usize> {
        let &Arr(_, ref vec) = value;
        let arrlen = try!(<X as NumCast>::from(vec.len()).ok_or(io::Error::new(io::ErrorKind::InvalidInput, "could not convert length of vector to Array length type")));
        let mut len = try!(<X as Endec>::encode(&arrlen, dst));
        for elem in vec{
            len += try!(<Y as Endec>::encode(elem, dst));
        }
        Ok(len)
    }

    fn decode(src: &mut Read) -> io::Result<Arr<X, Y>> {
        let len = try!(try!(<X as Endec>::decode(src)).to_usize().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "could not read length of vector from Array length type")));
        Ok(Arr::new(Vec::from_iter((0..len).map(|_| <Y as Endec>::decode(src).unwrap()))))
    }

}

impl <X:Endec, Y> Arr<X, Y> {
    pub fn new(vec: Vec<Y>) -> Arr<X, Y> {
        Arr(PhantomData, vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Endec;

    #[test]
    fn test_decode_string_array() {
        let buf = vec![0x00, 0x01, 0x00, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
        let Arr(_, arr) = <Arr<u16, String> as Endec>::decode(&mut &buf[..]).unwrap();

        assert_eq!(1, arr.len());
        assert_eq!("hello".to_string(), arr[0]);
    }

    #[test]
    fn test_decode_u16_array() {
        let buf = vec![0x00, 0x02, 0x00, 0x01, 0x00, 0x02];
        let Arr(_, arr) = <Arr<u16, u16> as Endec>::decode(&mut &buf[..]).unwrap();

        assert_eq!(2, arr.len());
        assert_eq!(vec![1,2], arr);
    }

    #[test]
    fn test_encode_string_array() {
        let expected = vec![0x00, 0x01, 0x00, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
        let mut vec: Vec<u8> = Vec::new();

        let encoded_len = <Arr<u16, String> as Endec>::encode(&Arr::new(vec!["hello".to_string()]), &mut vec).unwrap();

        assert_eq!(9, encoded_len);
        assert_eq!(expected, vec);
    }

    #[test]
    fn test_encode_u16_array() {
        let expected = vec![0x00, 0x02, 0x00, 0x01, 0x00, 0x02];
        let mut vec = Vec::new();

        let encoded_len = <Arr<u16, u16> as Endec>::encode(&Arr::new(vec![1,2]), &mut vec).unwrap();

        assert_eq!(6, encoded_len);
        assert_eq!(expected, vec);
    }
}
