#[macro_use] extern crate protobuilder;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use protobuilder::types::*;
use protobuilder::{PacketHeader};

struct HeaderEncoder;
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

protocol! {
    Testproto : HeaderEncoder => {
        0 => Message { a: u16, b: u16, c: String, d: Vec<u16> }
        1 => Msg { a: u16, b: u16, c: u16, d: u16, e: u16, f: u16 , g: u16, h: u16 }
    }
    //Otherproto: |x, y| -> [u8; 2] { [0u8, 2] }, |x: &mut Read| -> (u64, usize) { (0, 6) } => {
        //0 => Message { a: u16 }
    //}
}

fn main() {
    println!("Hello World");
}
