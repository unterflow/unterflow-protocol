extern crate byteorder;
#[macro_use]
extern crate unterflow_protocol_derive;

pub mod io;

use io::{FromBytes, HasBlockLength, ToBytes};

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
pub struct DataFrameHeader {
    length: u32,
    version: u8,
    flags: u8,
    type_id: u16,
    stream_id: u32,
}

impl DataFrameHeader {
    pub fn new(length: u32, version: u8, flags: u8, type_id: u16, stream_id: u32) -> Self {
        DataFrameHeader {
            length,
            version,
            flags,
            type_id,
            stream_id,
        }
    }

    pub fn aligned_length(&self) -> u32 {
        align(self.length + Self::block_length() as u32)
    }

    pub fn is_batch_begin(&self) -> bool {
        self.flags & 0b1000_0000 != 0
    }

    pub fn is_batch_end(&self) -> bool {
        self.flags & 0b0100_0000 != 0
    }

    pub fn is_failed(&self) -> bool {
        self.flags & 0b0010_0000 != 0
    }
}

pub fn align(value: u32) -> u32 {
    (value + 7) & !7
}


#[cfg(test)]
mod test {

    use super::*;
    use byteorder::{LittleEndian, WriteBytesExt};

    #[test]
    fn test_align() {
        assert_eq!(0, align(0));
        assert_eq!(8, align(1));
        assert_eq!(8, align(7));
        assert_eq!(8, align(8));
        assert_eq!(16, align(9));
        assert_eq!(192, align(190));
    }

    #[test]
    fn test_data_frame_header() {
        let mut buffer = vec![];

        buffer.write_u32::<LittleEndian>(10).unwrap();
        buffer.write_u8(11).unwrap();
        buffer.write_u8(224).unwrap();
        buffer.write_u16::<LittleEndian>(12).unwrap();
        buffer.write_u32::<LittleEndian>(13).unwrap();

        let header = DataFrameHeader::new(10, 11, 224, 12, 13);

        let mut bytes = vec![];
        header.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);

        assert_eq!(header,
                   DataFrameHeader::from_bytes(&mut &buffer[..]).unwrap());

        assert_eq!(12, DataFrameHeader::block_length());
        assert_eq!(24, header.aligned_length());
        assert!(header.is_batch_begin());
        assert!(header.is_batch_end());
        assert!(header.is_failed());
    }

}
