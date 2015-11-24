pub mod types;
extern crate num;

use std::io;
use std::io::prelude::*;

pub trait Endec:Sized{
    fn encode(value: &Self, dst: &mut Write) -> io::Result<usize>;
    fn decode(src: &mut Read) -> io::Result<Self>;
}

pub trait PacketHeader {
    fn write(id: u16, len: usize, dst: &mut Write) -> io::Result<usize>;
    fn read(src: &mut Read) -> io::Result<(u16, usize)>;
}

pub struct HeaderEncoder;
impl PacketHeader for HeaderEncoder {
    fn write(id: u16, len: usize, dst: &mut Write) -> io::Result<usize> {
        let mut header_len = try!(<u16 as Endec>::encode(&id, dst));
        header_len += try!(<usize as Endec>::encode(&len, dst));
        Ok(header_len)
    }
    fn read(src: &mut Read) -> io::Result<(u16, usize)> {
        let id:u16 = <u16 as Endec>::decode(src).unwrap();
        let len:usize = <usize as Endec>::decode(src).unwrap();
        Ok((id, len))
    }
}

#[macro_export]
macro_rules! packets {
    ($proto_name:ident, $header_endec:ty, $($id:expr => $name:ident { $($fname:ident: $fty:ty),* })+) => {
        #[derive(Debug,PartialEq)]
        enum $proto_name {
            $(
                $name { $($fname:$fty),* }
             ),+
        }

        impl Endec for $proto_name {

            fn encode(value: &Self, dst: &mut Write) -> io::Result<usize> {
                let mut len: usize = 0;
                let mut buf: Vec<u8> = Vec::new();

                match value { $(
                    &$proto_name::$name { $(ref $fname),* } => {
                        //TODO: try! is not the best thing here
                        $(len += try!(<$fty as Endec>::encode(&$fname, &mut buf));)*;
                        len += try!(<$header_endec as PacketHeader>::write($id, len, dst));
                        try!(dst.write(&buf));
                    }
                )+}

                Ok(len)
            }

            fn decode(src: &mut Read) -> io::Result<Self> {
                let (id, len) = try!(<$header_endec as PacketHeader>::read(src));
                let mut handle = src.take(len as u64);

                match id {
                    $(
                        $id => Ok($proto_name::$name {
                                    $(
                                        $fname:try!(<$fty as Endec>::decode(&mut handle))
                                    ),*
                                }),
                    )+
                        _ => Err(Error::new(ErrorKind::Other, "oh no!"))
                }
            }
        }
    }
}

#[macro_export]
macro_rules! protocol {
    ($($proto_name:ident : $header_endec:ty => {
        $($id:expr => $name:ident { $($fname:ident: $fty:ty),* })+})+) => {
            $(packets!($proto_name, $header_endec, $($id => $name { $($fname:$fty),* })+);)+
    };
    ($($proto_name:ident {
        $($id:expr => $name:ident { $($fname:ident: $fty:ty),* })+})+) => {
            $(packets!($proto_name, HeaderEncoder, $($id => $name { $($fname:$fty),* })+);)+
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::io::prelude::*;
    use std::io::{Error, ErrorKind};
    use types::arr::Arr;

    protocol! { Testproto : HeaderEncoder => {
        0 => Message { a: u16, b: u16, c: Arr<u16, String> }
        1 => Msg { a: u16, b: u16, c: u16, d: u16, e: u16, f: u16 , g: u16, h: u16 } }
    }

    protocol! { Otherproto {
            0 => Message { a: u16 } }
    }

    #[test]
    fn test_encode() {
        let x:Testproto = Testproto::Message { a: 10, b: 15, c: Arr::new(vec!["hi".to_string()]) };
        let mut buf:Vec<u8> = Vec::new();

        assert!(Testproto::encode(&x, &mut buf).unwrap() == 20);
    }

    #[test]
    fn test_decode() {
        let buf:Vec<u8> = vec![0u8, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 10, 0, 15, 0, 1, 0, 2, 104, 105];
        let msg = Testproto::decode(&mut &buf[..]).unwrap();
        assert!(Testproto::Message { a: 10, b: 15, c: Arr::<u16,String>::new(vec!["hi".to_string()]) } == msg);
    }
}
