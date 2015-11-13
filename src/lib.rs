use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};

pub trait Endec {
    type T;

    fn encoded_len(value: &Self::T) -> usize;

    fn encode(value: &Self::T, dst: &mut Write) -> io::Result<usize>;
    fn decode(src: &mut Read) -> io::Result<Self::T>;
}

impl Endec for u16 {
    type T = u16;

    fn encoded_len(value: &Self::T) -> usize {
        2
    }

    fn encode(value: &Self::T, dst: &mut Write) -> io::Result<usize> {
        let mut buf = [0u8; 2];
        buf[0] = (value >> 8 & 0xffu8 as u16) as u8;
        buf[1] = (value & 0xffu8 as u16) as u8;

        assert_eq!(dst.write(&buf).is_ok(), true);
        Ok(2)
    }

    fn decode(src: &mut Read) -> io::Result<Self::T> {
        let mut buf = [0u8; 2];
        assert_eq!(src.read(&mut buf).is_ok(), true);
        Ok(((buf[0] as u16) << 8 ) | (buf[1] as u16))
    }
}

macro_rules! packets {
    ($($id:expr => $name:ident { $($fname:ident: $fty:ty),* })+) => {
        #[derive(Debug)]
        enum Protocol {
            $(
                $name { $($fname:$fty),* }
             ),+
        }

        impl Endec for Protocol {
            type T = Protocol ;

            fn encoded_len(value: &Self::T) -> usize {
                let mut len: usize = 0;
                
                match value { $(
                    &Protocol::$name { $($fname),* } => {$(
                        //TODO: try! is not the best thing here
                        len += <$fty as Endec>::encoded_len(&$fname);
                    )*}
                )+}

                len
            }

            fn encode(value: &Self::T, dst: &mut Write) -> io::Result<usize> {
                let mut len: usize = 0;

                match value { $(
                    &Protocol::$name { $($fname),* } => {$(
                        //TODO: try! is not the best thing here
                        len += try!(<$fty as Endec>::encode(&$fname, dst));
                    )*}
                )+}

                Ok(len)
            }
            
            fn decode(src: &mut Read) -> io::Result<Self::T> {
                let id = 0;

                match id { 
                    $(
                        $id => Ok(Protocol::$name {
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

macro_rules! protocol {
    ($proto_name:ident : $header_func:expr => {
        $($id:expr => $name:ident { $($fname:ident: $fty:ty),* })+}) => {
            packets!($($id => $name { $($fname:$fty),* })+);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::io::prelude::*;
    use std::io::{Error, ErrorKind};

    protocol! {
        dummy : |x, y| -> [u8] { [0u8,2] } => {
            0 => Message { a: u16, b: u16 }
            1 => Msg { b: u16 }
        }
        //other : |x, y| -> [u8] { [0u8, 2] } => {
            //0 => Message { a: u16, b: u16 }
            //1 => Msg { b: u16 }
        //}
    }
    
    #[test]
    fn test_main() {


        let x:Protocol = Protocol::Message { a: 10, b: 15 };
        let mut buf:Vec<u8> = Vec::new();
        Protocol::encode(&x, &mut buf);
        println!("{:?}", buf);
        assert!(Protocol::encoded_len(&x) == 4);
    }
    
    
}
