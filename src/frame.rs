use io::{FromBytes, HasBlockLength, ToBytes};

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
#[enum_type = "u16"]
pub enum DataFrameType {
    Message,
    Padding,
}

impl Default for DataFrameType {
    fn default() -> Self {
        DataFrameType::Message
    }
}

#[derive(Debug, Default, PartialEq, FromBytes, ToBytes, HasBlockLength)]
pub struct DataFrameHeader {
    pub length: u32,
    pub version: u8,
    pub flags: u8,
    pub frame_type: DataFrameType,
    pub stream_id: u32,
}

impl DataFrameHeader {
    pub fn length(&self) -> usize {
        self.length as usize
    }

    pub fn aligned_length(&self) -> usize {
        align(self.length() + Self::block_length() as usize)
    }

    pub fn padding(&self) -> usize {
        self.aligned_length() - self.length() - Self::block_length() as usize
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

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
#[enum_type = "u16"]
pub enum TransportProtocol {
    RequestResponse,
    FullDuplexSingleMessage,
    ControlMessage,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
pub struct TransportHeader {
    pub protocol: TransportProtocol,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
pub struct RequestResponseHeader {
    pub request_id: u64,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
#[enum_type = "u32"]
pub enum ControlMessage {
    KeepAlive,
}

pub fn align(value: usize) -> usize {
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
        buffer.write_u16::<LittleEndian>(1).unwrap();
        buffer.write_u32::<LittleEndian>(13).unwrap();

        let header = DataFrameHeader {
            length: 10,
            version: 11,
            flags: 224,
            frame_type: DataFrameType::Padding,
            stream_id: 13,
        };

        let mut bytes = vec![];
        header.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);

        assert_eq!(
            header,
            DataFrameHeader::from_bytes(&mut &buffer[..]).unwrap()
        );

        assert_eq!(12, DataFrameHeader::block_length());
        assert_eq!(24, header.aligned_length());
        assert!(header.is_batch_begin());
        assert!(header.is_batch_end());
        assert!(header.is_failed());
    }

    #[test]
    fn test_transport_header() {
        let mut buffer = vec![];

        buffer.write_u16::<LittleEndian>(1).unwrap();

        let header = TransportHeader { protocol: TransportProtocol::FullDuplexSingleMessage };

        let mut bytes = vec![];
        header.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);

        assert_eq!(
            header,
            TransportHeader::from_bytes(&mut &buffer[..]).unwrap()
        );

        assert_eq!(2, TransportHeader::block_length());
        assert_eq!(TransportProtocol::FullDuplexSingleMessage, header.protocol)
    }

    #[test]
    fn test_request_response_header() {
        let mut buffer = vec![];

        buffer.write_u64::<LittleEndian>(256).unwrap();

        let header = RequestResponseHeader { request_id: 256 };

        let mut bytes = vec![];
        header.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);

        assert_eq!(
            header,
            RequestResponseHeader::from_bytes(&mut &buffer[..]).unwrap()
        );

        assert_eq!(8, RequestResponseHeader::block_length());
        assert_eq!(256, header.request_id);
    }
}
