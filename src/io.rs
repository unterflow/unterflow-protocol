use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};
use std::mem::size_of;

pub trait FromBytes {
    fn from_bytes(reader: &mut Read) -> Result<Self, io::Error> where Self: Sized;
}

pub trait ToBytes {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), io::Error>;
}

pub trait HasBlockLength {
    fn block_length() -> u16;
}

impl FromBytes for u8 {
    fn from_bytes(reader: &mut Read) -> Result<Self, io::Error> {
        reader.read_u8()
    }
}

impl ToBytes for u8 {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), io::Error> {
        writer.write_u8(*self)
    }
}

impl HasBlockLength for u8 {
    fn block_length() -> u16 {
        size_of::<u8>() as u16
    }
}

impl FromBytes for i8 {
    fn from_bytes(reader: &mut Read) -> Result<Self, io::Error> {
        reader.read_i8()
    }
}

impl ToBytes for i8 {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), io::Error> {
        writer.write_i8(*self)
    }
}

impl HasBlockLength for i8 {
    fn block_length() -> u16 {
        size_of::<i8>() as u16
    }
}

impl FromBytes for u16 {
    fn from_bytes(reader: &mut Read) -> Result<Self, io::Error> {
        reader.read_u16::<LittleEndian>()
    }
}

impl ToBytes for u16 {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), io::Error> {
        writer.write_u16::<LittleEndian>(*self)
    }
}

impl HasBlockLength for u16 {
    fn block_length() -> u16 {
        size_of::<u16>() as u16
    }
}

impl FromBytes for i16 {
    fn from_bytes(reader: &mut Read) -> Result<Self, io::Error> {
        reader.read_i16::<LittleEndian>()
    }
}

impl ToBytes for i16 {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), io::Error> {
        writer.write_i16::<LittleEndian>(*self)
    }
}

impl HasBlockLength for i16 {
    fn block_length() -> u16 {
        size_of::<i16>() as u16
    }
}

impl FromBytes for u32 {
    fn from_bytes(reader: &mut Read) -> Result<Self, io::Error> {
        reader.read_u32::<LittleEndian>()
    }
}

impl ToBytes for u32 {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), io::Error> {
        writer.write_u32::<LittleEndian>(*self)
    }
}

impl HasBlockLength for u32 {
    fn block_length() -> u16 {
        size_of::<u32>() as u16
    }
}

impl FromBytes for i32 {
    fn from_bytes(reader: &mut Read) -> Result<Self, io::Error> {
        reader.read_i32::<LittleEndian>()
    }
}

impl ToBytes for i32 {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), io::Error> {
        writer.write_i32::<LittleEndian>(*self)
    }
}

impl HasBlockLength for i32 {
    fn block_length() -> u16 {
        size_of::<i32>() as u16
    }
}

impl FromBytes for u64 {
    fn from_bytes(reader: &mut Read) -> Result<Self, io::Error> {
        reader.read_u64::<LittleEndian>()
    }
}

impl ToBytes for u64 {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), io::Error> {
        writer.write_u64::<LittleEndian>(*self)
    }
}

impl HasBlockLength for u64 {
    fn block_length() -> u16 {
        size_of::<u64>() as u16
    }
}

impl FromBytes for i64 {
    fn from_bytes(reader: &mut Read) -> Result<Self, io::Error> {
        reader.read_i64::<LittleEndian>()
    }
}

impl ToBytes for i64 {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), io::Error> {
        writer.write_i64::<LittleEndian>(*self)
    }
}

impl HasBlockLength for i64 {
    fn block_length() -> u16 {
        size_of::<i64>() as u16
    }
}

#[cfg(test)]
mod test {

    use super::{FromBytes, HasBlockLength, ToBytes};

    #[test]
    fn from_bytes_u8() {
        let mut buffer: &[u8] = &[1];
        assert_eq!(1u8, FromBytes::from_bytes(&mut buffer).unwrap());

        let mut empty: &[u8] = &[];
        assert!(u8::from_bytes(&mut empty).is_err());
    }

    #[test]
    fn to_bytes_u8() {
        let mut buffer = vec![];
        1u8.to_bytes(&mut buffer).unwrap();
        assert_eq!(vec![1], buffer);
    }

    #[test]
    fn from_bytes_i8() {
        let mut buffer: &[u8] = &[1];
        assert_eq!(1i8, FromBytes::from_bytes(&mut buffer).unwrap());

        let mut empty: &[u8] = &[];
        assert!(i8::from_bytes(&mut empty).is_err());
    }

    #[test]
    fn to_bytes_i8() {
        let mut buffer = vec![];
        1i8.to_bytes(&mut buffer).unwrap();
        assert_eq!(vec![1], buffer);
    }

    #[test]
    fn from_bytes_u16() {
        let mut buffer: &[u8] = &[0, 1];
        assert_eq!(256u16, FromBytes::from_bytes(&mut buffer).unwrap());

        let mut empty: &[u8] = &[];
        assert!(u16::from_bytes(&mut empty).is_err());
    }

    #[test]
    fn to_bytes_u16() {
        let mut buffer = vec![];
        256u16.to_bytes(&mut buffer).unwrap();
        assert_eq!(vec![0, 1], buffer);
    }

    #[test]
    fn from_bytes_i16() {
        let mut buffer: &[u8] = &[0, 1];
        assert_eq!(256i16, FromBytes::from_bytes(&mut buffer).unwrap());

        let mut empty: &[u8] = &[];
        assert!(i16::from_bytes(&mut empty).is_err());
    }

    #[test]
    fn to_bytes_i16() {
        let mut buffer = vec![];
        256i16.to_bytes(&mut buffer).unwrap();
        assert_eq!(vec![0, 1], buffer);
    }

    #[test]
    fn from_bytes_u32() {
        let mut buffer: &[u8] = &[0, 0, 1, 0];
        assert_eq!(65536u32, FromBytes::from_bytes(&mut buffer).unwrap());

        let mut empty: &[u8] = &[];
        assert!(u32::from_bytes(&mut empty).is_err());
    }

    #[test]
    fn to_bytes_u32() {
        let mut buffer = vec![];
        65536u32.to_bytes(&mut buffer).unwrap();
        assert_eq!(vec![0, 0, 1, 0], buffer);
    }

    #[test]
    fn from_bytes_i32() {
        let mut buffer: &[u8] = &[0, 0, 1, 0];
        assert_eq!(65536i32, FromBytes::from_bytes(&mut buffer).unwrap());

        let mut empty: &[u8] = &[];
        assert!(i32::from_bytes(&mut empty).is_err());
    }

    #[test]
    fn to_bytes_i32() {
        let mut buffer = vec![];
        65536i32.to_bytes(&mut buffer).unwrap();
        assert_eq!(vec![0, 0, 1, 0], buffer);
    }

    #[test]
    fn from_bytes_u64() {
        let mut buffer: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 0];
        assert_eq!(16777216u64, FromBytes::from_bytes(&mut buffer).unwrap());

        let mut empty: &[u8] = &[];
        assert!(u64::from_bytes(&mut empty).is_err());
    }

    #[test]
    fn to_bytes_u64() {
        let mut buffer = vec![];
        16777216u64.to_bytes(&mut buffer).unwrap();
        assert_eq!(vec![0, 0, 0, 1, 0, 0, 0, 0], buffer);
    }

    #[test]
    fn from_bytes_i64() {
        let mut buffer: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 0];
        assert_eq!(16777216i64, FromBytes::from_bytes(&mut buffer).unwrap());

        let mut empty: &[u8] = &[];
        assert!(i64::from_bytes(&mut empty).is_err());
    }

    #[test]
    fn to_bytes_i64() {
        let mut buffer = vec![];
        16777216i64.to_bytes(&mut buffer).unwrap();
        assert_eq!(vec![0, 0, 0, 1, 0, 0, 0, 0], buffer);
    }

    #[test]
    fn has_block_length() {
        assert_eq!(1, u8::block_length());
        assert_eq!(1, i8::block_length());
        assert_eq!(2, u16::block_length());
        assert_eq!(2, i16::block_length());
        assert_eq!(4, u32::block_length());
        assert_eq!(4, i32::block_length());
        assert_eq!(8, u64::block_length());
        assert_eq!(8, i64::block_length());
    }


}
