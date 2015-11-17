pub mod types;
extern crate num;

use std::io;
use std::io::prelude::*;

pub trait Endec {
    type T;

    fn encode(value: &Self::T, dst: &mut Write) -> io::Result<usize>;
    fn decode(src: &mut Read) -> io::Result<Self::T>;
}

pub trait PacketHeader {
    fn write(id: u16, len: usize, dst: &mut Write) -> io::Result<usize>;
    fn read(src: &mut Read) -> io::Result<(u16, usize)>;
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
            type T = $proto_name;

            fn encode(value: &Self::T, dst: &mut Write) -> io::Result<usize> {
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
            
            fn decode(src: &mut Read) -> io::Result<Self::T> {
                let (id, len) = try!(<$header_endec as PacketHeader>::read(src));

                match id { 
                    $(
                        $id => Ok($proto_name::$name {
                                    $(
                                        $fname:try!(<$fty as Endec>::decode(src))
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
    }
}


//#[cfg(test)]
//mod tests {
    //use super::*;
    //use std::io;
    //use std::io::prelude::*;
    //use std::io::{Error, ErrorKind};

    //struct HeaderEncoder;
    //impl PacketHeader for HeaderEncoder {
        //fn write(id: u16, len: usize, dst: &mut Write) -> io::Result<usize> {
            //let mut hdr_len= try!(<u16 as Endec>::encode(&id, dst));
            //hdr_len += try!(<usize as Endec>::encode(&len, dst));
            //Ok(hdr_len)
        //}
        //fn read(src: &mut Read) -> io::Result<(u16, usize)> {
            //let id:u16 = <u16 as Endec>::decode(src).unwrap();
            //let len:usize = <usize as Endec>::decode(src).unwrap();
            //Ok((id, len))
        //}
    //}

    //protocol! {
        //Testproto : HeaderEncoder => {
            //0 => Message { a: u16, b: u16 }
            //1 => Msg { a: u16, b: u16, c: u16, d: u16, e: u16, f: u16 , g: u16, h: u16 }
        //}
        ////Otherproto: |x, y| -> [u8; 2] { [0u8, 2] }, |x: &mut Read| -> (u64, usize) { (0, 6) } => {
            ////0 => Message { a: u16 }
        ////}
    //}
    
    //#[test]
    //fn test_encode() {
        //let x:Testproto = Testproto::Message { a: 10, b: 15 };
        //let mut buf:Vec<u8> = Vec::new();
        //assert!(Testproto::encode(&x, &mut buf).unwrap() == 8);
    //}
    
    //#[test]
    //fn test_decode() {
        //let buf:Vec<u8> = vec![0u8, 0, 0, 4, 0, 10, 0, 15];
        //let msg = Testproto::decode(&mut &buf[..]).unwrap();
        //assert!(Testproto::Message { a: 10, b: 15 } == msg);
    //}
//}
