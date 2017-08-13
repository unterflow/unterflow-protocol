use io::{FromBytes, HasBlockLength, ToBytes};

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
#[enum_type = "u16"]
pub enum DataFrameType {
    Message,
    Padding,
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
pub struct DataFrameHeader {
    length: u32,
    version: u8,
    flags: u8,
    frame_type: DataFrameType,
    stream_id: u32,
}

impl DataFrameHeader {
    pub fn new(length: u32, version: u8, flags: u8, frame_type: DataFrameType, stream_id: u32) -> Self {
        DataFrameHeader {
            length,
            version,
            flags,
            frame_type,
            stream_id,
        }
    }

    pub fn length(&self) -> usize {
        self.length as usize
    }

    pub fn aligned_length(&self) -> usize {
        align(self.length() + Self::block_length() as usize)
    }

    pub fn padding(&self) -> usize {
        self.aligned_length() - self.length() - Self::block_length() as usize
    }

    pub fn frame_type(&self) -> &DataFrameType {
        &self.frame_type
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
    protocol: TransportProtocol,
}

impl TransportHeader {
    pub fn new(protocol: TransportProtocol) -> Self {
        TransportHeader { protocol }
    }

    pub fn protocol(&self) -> &TransportProtocol {
        &self.protocol
    }
}

#[derive(Debug, PartialEq, FromBytes, ToBytes, HasBlockLength)]
pub struct RequestResponseHeader {
    request_id: u64,
}

impl RequestResponseHeader {
    pub fn new(request_id: u64) -> Self {
        RequestResponseHeader { request_id }
    }

    pub fn request_id(&self) -> u64 {
        self.request_id
    }
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

        let header = DataFrameHeader::new(10, 11, 224, DataFrameType::Padding, 13);

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

    #[test]
    fn test_transport_header() {
        let mut buffer = vec![];

        buffer.write_u16::<LittleEndian>(1).unwrap();

        let header = TransportHeader::new(TransportProtocol::FullDuplexSingleMessage);

        let mut bytes = vec![];
        header.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);

        assert_eq!(header,
                   TransportHeader::from_bytes(&mut &buffer[..]).unwrap());

        assert_eq!(2, TransportHeader::block_length());
        assert_eq!(&TransportProtocol::FullDuplexSingleMessage,
                   header.protocol())
    }

    #[test]
    fn test_request_response_header() {
        let mut buffer = vec![];

        buffer.write_u64::<LittleEndian>(256).unwrap();

        let header = RequestResponseHeader::new(256);

        let mut bytes = vec![];
        header.to_bytes(&mut bytes).unwrap();

        assert_eq!(buffer, bytes);

        assert_eq!(header,
                   RequestResponseHeader::from_bytes(&mut &buffer[..]).unwrap());

        assert_eq!(8, RequestResponseHeader::block_length());
        assert_eq!(256, header.request_id());
    }

}
