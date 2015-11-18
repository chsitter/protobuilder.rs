use super::Endec;
use std::io;
use std::io::{Read, Write};
use types::util::{decode_u16, encode_u16, decode_i32, encode_i32, decode_usize, encode_usize};

macro_rules! impl_endec{
    ($name:ty, $enc_func:ident, $dec_func:ident) => {
        impl Endec for $name {

            fn encode(value: &$name, dst: &mut Write) -> io::Result<usize> {
                $enc_func(value, dst)
            }

            fn decode(src: &mut Read) -> io::Result<$name> {
                $dec_func(src)
            }
        }
    }
}

impl_endec!(u16, encode_u16, decode_u16);
impl_endec!(i32, encode_i32, decode_i32);
impl_endec!(usize, encode_usize, decode_usize); 
