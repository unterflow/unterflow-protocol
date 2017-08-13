use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rmp_serde::{Deserializer, Serializer};
use rmp_serde::encode::StructMapWriter;
use serde::{Deserialize, Serialize};
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

pub trait Message {
    fn template_id() -> u16;
    fn schema_id() -> u16;
    fn version() -> u16;
}

pub trait HasData {
    fn data(&self) -> &Data;
}

pub trait FromData {
    fn from_data<H: HasData>(has_data: &H) -> Result<Self, io::Error> where Self: Sized;
}

pub trait ToData {
    fn to_data(&self) -> Result<Data, io::Error>;
}

pub trait HasMessageLength {
    fn message_length(&self) -> u32;
}

macro_rules! impl_has_message_length {
    ($t:ty) => (
        impl HasMessageLength for $t {
            fn message_length(&self) -> u32 {
                ::std::mem::size_of::<$t>() as u32
            }
        }
    )
}

impl_has_message_length!(u8);
impl_has_message_length!(i8);
impl_has_message_length!(u16);
impl_has_message_length!(i16);
impl_has_message_length!(u32);
impl_has_message_length!(i32);
impl_has_message_length!(u64);
impl_has_message_length!(i64);

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

#[derive(Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Data(Vec<u8>);

impl ::std::ops::Deref for Data {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

impl From<Data> for Vec<u8> {
    fn from(data: Data) -> Self {
        data.0
    }
}

impl From<Vec<u8>> for Data {
    fn from(vec: Vec<u8>) -> Self {
        Data(vec)
    }
}

impl FromBytes for Data {
    fn from_bytes(reader: &mut Read) -> Result<Self, io::Error> {
        let length = reader.read_u16::<LittleEndian>()?;
        let mut buffer = Vec::with_capacity(length as usize);
        let mut handle = reader.take(length as u64);
        handle.read_to_end(&mut buffer)?;
        Ok(Data(buffer))
    }
}

impl ToBytes for Data {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), io::Error> {
        let length = self.0.len() as u16;
        writer.write_u16::<LittleEndian>(length)?;
        writer.write_all(&self.0)
    }
}

impl HasData for Data {
    fn data(&self) -> &Data {
        self
    }
}

impl HasMessageLength for Data {
    fn message_length(&self) -> u32 {
        (size_of::<u16>() + self.0.len()) as u32
    }
}

impl FromBytes for String {
    fn from_bytes(reader: &mut Read) -> Result<Self, io::Error> {
        let buffer: Data = FromBytes::from_bytes(reader)?;

        String::from_utf8(buffer.to_vec()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl ToBytes for String {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), io::Error> {
        let length = self.len() as u16;
        writer.write_u16::<LittleEndian>(length)?;
        writer.write_all(self.as_bytes())
    }
}

impl HasMessageLength for String {
    fn message_length(&self) -> u32 {
        (size_of::<u16>() + self.as_bytes().len()) as u32
    }
}

impl<T: FromBytes> FromBytes for Vec<T> {
    fn from_bytes(reader: &mut Read) -> Result<Self, io::Error> {
        let _block_length = reader.read_u16::<LittleEndian>()?;
        let num_in_group = reader.read_u8()?;
        let mut group: Vec<T> = Vec::with_capacity(num_in_group as usize);
        for _ in 0..num_in_group {
            group.push(T::from_bytes(reader)?);
        }
        Ok(group)
    }
}

impl<T: ToBytes + HasBlockLength> ToBytes for Vec<T> {
    fn to_bytes(&self, writer: &mut Write) -> Result<(), io::Error> {
        writer.write_u16::<LittleEndian>(T::block_length())?;

        let length = self.len() as u8;
        writer.write_u8(length)?;

        for element in self {
            element.to_bytes(writer)?;
        }

        Ok(())
    }
}

impl<'d, T> FromData for T
    where T: Deserialize<'d>
{
    fn from_data<H: HasData>(has_data: &H) -> Result<Self, io::Error> {
        let reader: &[u8] = has_data.data();
        let mut de = Deserializer::new(reader);

        Deserialize::deserialize(&mut de).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl<T> ToData for T
    where T: Serialize
{
    fn to_data(&self) -> Result<Data, io::Error> {
        let mut buffer = Vec::new();
        self.serialize(&mut Serializer::with(&mut buffer, StructMapWriter))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(Data(buffer))
    }
}

impl<T: HasMessageLength> HasMessageLength for Vec<T> {
    fn message_length(&self) -> u32 {
        let length: u32 = self.into_iter().map(HasMessageLength::message_length).sum();
        (size_of::<u16>() + size_of::<u8>()) as u32 + length
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use byteorder::{LittleEndian, WriteBytesExt};
    use io::Write;

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
    fn from_bytes_string() {
        let expected = "foobar".to_string();

        let mut buffer = vec![];
        buffer.write_u16::<LittleEndian>(6).unwrap();
        buffer.write_all(expected.as_bytes()).unwrap();

        assert_eq!(expected, String::from_bytes(&mut &buffer[..]).unwrap());
    }

    #[test]
    fn to_bytes_string() {
        let s = "foobar".to_string();

        let mut buffer = vec![];
        s.to_bytes(&mut buffer).unwrap();

        assert_eq!(vec![6, 0, 102, 111, 111, 98, 97, 114], buffer);
    }

    #[test]
    fn from_bytes_data() {
        let expected = Data::from(vec![1, 2, 3, 4]);

        let mut buffer = vec![];
        buffer.write_u16::<LittleEndian>(4).unwrap();
        buffer.write_all(&expected).unwrap();

        assert_eq!(expected, Data::from_bytes(&mut &buffer[..]).unwrap());
    }

    #[test]
    fn to_bytes_data() {
        let data = Data::from(vec![1, 2, 3, 4]);
        let mut buffer = vec![];

        data.to_bytes(&mut buffer).unwrap();

        assert_eq!(vec![4, 0, 1, 2, 3, 4], buffer);
    }

    #[test]
    fn from_bytes_collection() {
        let expected = vec![1u32, 2u32, 3u32, 4u32];

        let mut buffer = vec![];
        buffer
            .write_u16::<LittleEndian>(u32::block_length())
            .unwrap();
        buffer.write_u8(expected.len() as u8).unwrap();
        buffer.write_u32::<LittleEndian>(1).unwrap();
        buffer.write_u32::<LittleEndian>(2).unwrap();
        buffer.write_u32::<LittleEndian>(3).unwrap();
        buffer.write_u32::<LittleEndian>(4).unwrap();

        assert_eq!(expected, Vec::<u32>::from_bytes(&mut &buffer[..]).unwrap());
    }

    #[test]
    fn to_bytes_collection() {
        let c = vec![1u16, 2u16, 3u16, 4u16];

        let mut buffer = vec![];
        c.to_bytes(&mut buffer).unwrap();

        assert_eq!(vec![2, 0, 4, 1, 0, 2, 0, 3, 0, 4, 0], buffer);
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

    #[test]
    fn from_data() {
        let data = Data(vec![0x92, 0x0c, 0xa3, 0x61, 0x62, 0x63]);
        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        struct Foo {
            a: u32,
            b: String,
        }

        let expected = Foo {
            a: 12,
            b: "abc".to_string(),
        };

        assert_eq!(expected, Foo::from_data(&data).unwrap());
    }

    #[test]
    fn to_data() {
        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        struct Foo {
            a: u32,
            b: String,
        }

        let foo = Foo {
            a: 12,
            b: "abc".to_string(),
        };

        assert_eq!(Data(vec![0x82, 0xa1, 0x61, 0x0c, 0xa1, 0x62, 0xa3, 0x61, 0x62, 0x63]),
                   foo.to_data().unwrap());
    }

}
