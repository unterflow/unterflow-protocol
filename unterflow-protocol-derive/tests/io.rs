extern crate unterflow_protocol;
#[macro_use]
extern crate unterflow_protocol_derive;

use unterflow_protocol::io::{FromBytes, HasBlockLength, ToBytes};


#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
enum Enum {
    A,
    B,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
#[enum_type = "u32"]
enum EnumWithType {
    A,
    B,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
enum EnumWithCustomValues {
    A,
    B = 16,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
#[enum_type = "u16"]
enum EnumWithTypeAndCustomValues {
    A,
    B = 16,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
struct Struct {
    a: u8,
    b: i8,
    c: u16,
    d: i16,
    e: u32,
    f: i32,
    g: u64,
    h: i64,
    i: Enum,
    j: EnumWithType,
    k: EnumWithCustomValues,
    l: EnumWithTypeAndCustomValues,
}

#[test]
fn from_bytes_enum() {
    let mut buffer: &[u8] = &[1, 1, 0, 0, 0, 16, 16, 0];

    assert_eq!(Enum::B, FromBytes::from_bytes(&mut buffer).unwrap());
    assert_eq!(EnumWithType::B, FromBytes::from_bytes(&mut buffer).unwrap());
    assert_eq!(EnumWithCustomValues::B,
               FromBytes::from_bytes(&mut buffer).unwrap());
    assert_eq!(EnumWithTypeAndCustomValues::B,
               FromBytes::from_bytes(&mut buffer).unwrap());
}

#[test]
fn from_bytes_enum_error() {
    let mut buffer: &[u8] = &[2, 2, 0, 0, 0, 1, 1, 0];

    assert!(Enum::from_bytes(&mut buffer).is_err());
    assert!(EnumWithType::from_bytes(&mut buffer).is_err());
    assert!(EnumWithType::from_bytes(&mut buffer).is_err());
    assert!(EnumWithCustomValues::from_bytes(&mut buffer).is_err());
    assert!(EnumWithTypeAndCustomValues::from_bytes(&mut buffer).is_err());
}


#[test]
fn from_bytes_struct() {
    let mut buffer: &[u8] = &[1, 2, 0, 1, 0, 2, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1, 1, 0, 0, 0, 16, 16, 0];

    let expected = Struct {
        a: 1,
        b: 2,
        c: 256,
        d: 512,
        e: 65536,
        f: 131072,
        g: 16777216,
        h: 33554432,
        i: Enum::B,
        j: EnumWithType::B,
        k: EnumWithCustomValues::B,
        l: EnumWithTypeAndCustomValues::B,
    };

    assert_eq!(expected, Struct::from_bytes(&mut buffer).unwrap());
}

#[test]
fn to_bytes_struct() {
    let expected = vec![1, 2, 0, 1, 0, 2, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 1, 1, 0, 0, 0, 16, 16, 0];

    let s = Struct {
        a: 1,
        b: 2,
        c: 256,
        d: 512,
        e: 65536,
        f: 131072,
        g: 16777216,
        h: 33554432,
        i: Enum::B,
        j: EnumWithType::B,
        k: EnumWithCustomValues::B,
        l: EnumWithTypeAndCustomValues::B,
    };

    let mut buffer = vec![];

    s.to_bytes(&mut buffer).unwrap();

    assert_eq!(expected, buffer);
}

#[test]
fn has_block_length() {
    assert_eq!(1, Enum::block_length());
    assert_eq!(4, EnumWithType::block_length());
    assert_eq!(1, EnumWithCustomValues::block_length());
    assert_eq!(2, EnumWithTypeAndCustomValues::block_length());
    assert_eq!(38, Struct::block_length());
}
