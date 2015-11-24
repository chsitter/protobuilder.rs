#[macro_use] extern crate protobuilder;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use protobuilder::Endec;
use protobuilder::{PacketHeader, HeaderEncoder};
use protobuilder::types::arr::Arr;

protocol! { Testproto {
    0 => Message { a: u16, b: u16, c: String, d: Arr<u16, u16> }
    1 => Msg { a: u16, b: u16, c: u16, d: u16, e: u16, f: u16 , g: u16, h: u16 } }
}

fn main() {
    println!("Hello World");
}
