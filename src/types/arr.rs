use super::Endec;
use std::marker::PhantomData;
use std::io;
use std::io::prelude::*;
use num::{NumCast, ToPrimitive};
use std::iter::FromIterator;

#[derive(PartialEq, Debug)]
pub struct Arr<X, Y>(PhantomData<X>, PhantomData<Y>);

//impl<X: Endec, Y: Endec> Endec for Arr<X, Y> where X::T: NumCast {
    //type T = Vec<Y::T>;

    //fn encode(value: &Vec<Y::T>, dst: &mut Write) -> io::Result<usize> {
        //let mut arrlen = try!(<X::T as NumCast>::from(value.len()).ok_or(()));
        //let mut len = try!(<X as Endec>::encode(&arrlen, dst));
        //for elem in value {
            //len += try!(<Y as Endec>::encode(elem, dst));
        //}
        //Ok(len)
    //}

    //fn decode(src: &mut Read) -> io::Result<Vec<Y::T>> {
        //let len = try!(try!(<X as Endec>::decode(src)).to_usize().ok_or(()));
        //Ok(Vec::from_iter((0..len).map(|_| <Y as Endec>::decode(src).unwrap())))
    //}
//}


//#[cfg(test)]
//mod tests {
    //use super::Endec;
    //use super::*;

    //#[test]
    //fn test_decode_string_array() {
        //let buf = vec![0x00, 0x01, 0x00, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
        //let arr = <Arr<u16, String> as Endec>::proto_decode(&mut &buf[..]).unwrap();

        //assert_eq!(1, arr.len());
        //assert_eq!("hello".to_string(), arr[0]);
    //}
    
    //#[test]
    //fn test_decode_u16_array() {
        //let buf = vec![0x00, 0x02, 0x00, 0x01, 0x00, 0x02];
        //let arr = <Arr<u16, u16> as Endec>::proto_decode(&mut &buf[..]).unwrap();

        //assert_eq!(2, arr.len());
        //assert_eq!(vec![1,2], arr);
    //}
    
    //#[test]
    //fn test_encode_string_array() {
        //let expected = vec![0x00, 0x01, 0x00, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
        //let mut vec: Vec<u8> = Vec::new();
        
        //<Arr<u16, String> as Endec>::encode(&vec!["hello".to_string()], &mut vec);

        //assert_eq!(9, <Arr<u16, String> as Endec>::proto_len(&vec!["hello".to_string()]));
        //assert_eq!(expected, vec);
    //}
    
    //#[test]
    //fn test_encode_u16_array() {
        //let expected = vec![0x00, 0x02, 0x00, 0x01, 0x00, 0x02];
        //let mut vec = Vec::new();
        
        //<Arr<u16,u16> as Endec>::encode(&vec![1,2], &mut vec);
        
        //assert_eq!(6, <Arr<u16,u16> as Endec>::proto_len(&vec![1,2]));
        //assert_eq!(expected, vec);
    //}
//}
