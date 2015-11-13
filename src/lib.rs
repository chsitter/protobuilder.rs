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

impl Endec for usize {
    type T = usize;

    fn encoded_len(value: &Self::T) -> usize {
        2
    }

    fn encode(value: &Self::T, dst: &mut Write) -> io::Result<usize> {
        let mut buf = [0u8; 2];
        buf[0] = (value >> 8 & 0xffu8 as usize) as u8;
        buf[1] = (value & 0xffu8 as usize) as u8;

        assert_eq!(dst.write(&buf).is_ok(), true);
        Ok(2)
    }

    fn decode(src: &mut Read) -> io::Result<Self::T> {
        let mut buf = [0u8; 2];
        assert_eq!(src.read(&mut buf).is_ok(), true);
        Ok(((buf[0] as usize) << 8 ) | (buf[1] as usize))
    }
}

macro_rules! packets {
    ($proto_name:ident, $header_func:expr, $header_dec_func:expr, $($id:expr => $name:ident { $($fname:ident: $fty:ty),* })+) => {
        #[derive(Debug)]
        enum $proto_name {
            $(
                $name { $($fname:$fty),* }
             ),+
        }

        impl Endec for $proto_name {
            type T = $proto_name;

            fn encoded_len(value: &Self::T) -> usize {
                let mut len: usize = 0;
                
                match value { $(
                    &$proto_name::$name { $($fname),* } => {$(
                        len += <$fty as Endec>::encoded_len(&$fname);
                    )*}
                )+}

                len
            }

            fn encode(value: &Self::T, dst: &mut Write) -> io::Result<usize> {
                let mut len: usize = 0;
                

                match value { $(
                    &$proto_name::$name { $($fname),* } => {
                        //TODO: try! is not the best thing here
                        len += try!(dst.write(&$header_func($id, <$proto_name as Endec>::encoded_len(value))));
                        $(len += try!(<$fty as Endec>::encode(&$fname, dst));)*
                    }
                )+}

                Ok(len)
            }
            
            fn decode(src: &mut Read) -> io::Result<Self::T> {
                let mut id = 0;
                let mut len = 0;
                {
                let (id, len) = $header_dec_func(src);
                }

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

macro_rules! protocol {
    ($($proto_name:ident : $header_func:expr, $header_dec_func:expr => {
        $($id:expr => $name:ident { $($fname:ident: $fty:ty),* })+})+) => {
            $(packets!($proto_name, $header_func, $header_dec_func, $($id => $name { $($fname:$fty),* })+);)+
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::io::prelude::*;
    use std::io::{Error, ErrorKind};

    protocol! {
        Testproto : |x, y| -> Vec<u8> { 
            let mut buf:Vec<u8>= Vec::new();
            <u16 as Endec>::encode(&x, &mut buf);
            <usize as Endec>::encode(&y, &mut buf);
            buf
        }, |x: &mut Read| -> (u16, usize) { 
            let a:u16 = <u16 as Endec>::decode(x).unwrap();
            let b:usize = <usize as Endec>::decode(x).unwrap();
            (a, b)
        } => {
            0 => Message { a: u16, b: u16 }
            1 => Msg { b: u16 }
        }
        Otherproto: |x, y| -> [u8; 2] { [0u8, 2] }, |x: &mut Read| -> (u64, usize) { (0, 6) } => {
            0 => Message { a: u16 }
        }
    }
    
    #[test]
    fn test_encode() {
        let x:Testproto = Testproto::Message { a: 10, b: 15 };
        //let y:Otherproto= Otherproto::Message { a: 10 };
        let mut buf:Vec<u8> = Vec::new();
        Testproto::encode(&x, &mut buf);
        println!("{:?}", buf);
        assert!(Testproto::encoded_len(&x) == 4);
    }
    
    #[test]
    fn test_decode() {
        let buf:Vec<u8> = vec![0u8, 0, 0, 4, 0, 10, 0, 15];
        let msg = Testproto::decode(&mut &buf[..]);
        println!("{:?}", msg);
        //assert!(Testproto::Message { a: 10, b: 15 } == msg);
    }


}
