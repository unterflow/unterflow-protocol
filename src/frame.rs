use std::io::{Error, ErrorKind};
use std::mem;
use std::slice;

const DATA_FRAME_ALIGNMENT: usize = 8;

macro_rules! size_of {
($($t:ty), *) => {{
    let mut length = 0;
    $(length += mem::size_of::<$t>();); *
    length as u32
}};
}

#[derive(Debug)]
pub enum Frame<'f> {
    RequestResponse(RequestResponseFrame<'f>),
    FullDuplexSingleMessage(FullDuplexSingleMessageFrame<'f>),
    ControlMessage(ControlMessageFrame<'f>),
    Padding(PaddingFrame<'f>),
}

impl<'f> Frame<'f> {
    pub fn decode(data: &'f [u8]) -> Result<Self, Error> {
        let mut decoder = Decoder::new(data);

        let data_frame_header = decoder.read_type::<DataFrameHeader>()?;

        match data_frame_header.frame_type {
            DataFrameType::Message => {
                // truncate to message length
                decoder.truncate(data_frame_header.length())?;

                let transport_header = decoder.read_type::<TransportHeader>()?;
                match transport_header.protocol {
                    TransportProtocol::RequestResponse => {
                        let request_response_header = decoder.read_type::<RequestResponseHeader>()?;
                        let message = decoder.remaining();

                        Ok(Frame::RequestResponse(RequestResponseFrame {
                            data_frame_header,
                            transport_header,
                            request_response_header,
                            message,
                        }))
                    }
                    TransportProtocol::FullDuplexSingleMessage => {
                        let message = decoder.remaining();

                        Ok(Frame::FullDuplexSingleMessage(
                            FullDuplexSingleMessageFrame {
                                data_frame_header,
                                transport_header,
                                message,
                            },
                        ))
                    }
                    TransportProtocol::ControlMessage => {
                        let message_type = decoder.read_type::<ControlMessageType>()?;
                        Ok(Frame::ControlMessage(ControlMessageFrame {
                            data_frame_header,
                            transport_header,
                            message_type,
                        }))
                    }
                }
            }
            DataFrameType::Padding => Ok(Frame::Padding(PaddingFrame { data_frame_header })),
        }

    }
}

#[derive(Debug)]
pub struct RequestResponseFrame<'f> {
    data_frame_header: &'f DataFrameHeader,
    transport_header: &'f TransportHeader,
    request_response_header: &'f RequestResponseHeader,
    message: &'f [u8],
}

pub fn request_response(version: u8, flags: u8, stream_id: u32, request_id: u64, message: &[u8]) -> Result<Vec<u8>, Error> {
    let length = size_of!(DataFrameHeader, TransportHeader, RequestResponseHeader) + message.len() as u32;

    let data_frame_header = DataFrameHeader {
        length,
        version,
        flags,
        frame_type: DataFrameType::Message,
        stream_id,
    };

    let transport_header = TransportHeader { protocol: TransportProtocol::RequestResponse };

    let request_response_header = RequestResponseHeader { request_id };

    let mut buffer = vec![0; data_frame_header.aligned_length()];

    {
        let mut encoder = Encoder::new(&mut buffer);
        encoder.write_type(&data_frame_header)?;
        encoder.write_type(&transport_header)?;
        encoder.write_type(&request_response_header)?;
        encoder.write(message)?;
    }

    Ok(buffer)
}

#[derive(Debug)]
pub struct FullDuplexSingleMessageFrame<'f> {
    data_frame_header: &'f DataFrameHeader,
    transport_header: &'f TransportHeader,
    message: &'f [u8],
}

pub fn full_duplex_single_message(version: u8, flags: u8, stream_id: u32, message: &[u8]) -> Result<Vec<u8>, Error> {
    let length = size_of!(DataFrameHeader, TransportHeader) + message.len() as u32;

    let data_frame_header = DataFrameHeader {
        length,
        version,
        flags,
        frame_type: DataFrameType::Message,
        stream_id,
    };

    let transport_header = TransportHeader { protocol: TransportProtocol::FullDuplexSingleMessage };

    let mut buffer = vec![0; data_frame_header.aligned_length()];

    {
        let mut encoder = Encoder::new(&mut buffer);
        encoder.write_type(&data_frame_header)?;
        encoder.write_type(&transport_header)?;
        encoder.write(message)?;
    }

    Ok(buffer)
}

#[derive(Debug)]
pub struct ControlMessageFrame<'f> {
    data_frame_header: &'f DataFrameHeader,
    transport_header: &'f TransportHeader,
    message_type: &'f ControlMessageType,
}

pub fn control_message(version: u8, flags: u8, stream_id: u32, message_type: &ControlMessageType) -> Result<Vec<u8>, Error> {
    let length = size_of!(DataFrameHeader, TransportHeader, ControlMessageType);

    let data_frame_header = DataFrameHeader {
        length,
        version,
        flags,
        frame_type: DataFrameType::Message,
        stream_id,
    };

    let transport_header = TransportHeader { protocol: TransportProtocol::ControlMessage };

    let mut buffer = vec![0; data_frame_header.aligned_length()];

    {
        let mut encoder = Encoder::new(&mut buffer);
        encoder.write_type(&data_frame_header)?;
        encoder.write_type(&transport_header)?;
        encoder.write_type(message_type)?;
    }

    Ok(buffer)
}

#[derive(Debug)]
pub struct PaddingFrame<'f> {
    data_frame_header: &'f DataFrameHeader,
}

pub fn padding(version: u8, flags: u8, stream_id: u32, padding: u32) -> Result<Vec<u8>, Error> {
    let length = size_of!(DataFrameHeader) + padding;

    let data_frame_header = DataFrameHeader {
        length,
        version,
        flags,
        frame_type: DataFrameType::Padding,
        stream_id,
    };

    let mut buffer = vec![0; data_frame_header.aligned_length()];

    {
        let mut encoder = Encoder::new(&mut buffer);
        encoder.write_type(&data_frame_header)?;
        encoder.write(&vec![0; padding as usize])?;
    }

    Ok(buffer)
}

#[derive(Debug)]
#[repr(C, packed)]
struct DataFrameHeader {
    length: u32,
    version: u8,
    flags: u8,
    frame_type: DataFrameType,
    stream_id: u32,
}

impl DataFrameHeader {
    fn length(&self) -> usize {
        self.length as usize
    }

    fn aligned_length(&self) -> usize {
        align(self.length(), DATA_FRAME_ALIGNMENT)
    }
}

fn align(value: usize, alignment: usize) -> usize {
    (value + (alignment - 1)) & !(alignment - 1)
}

#[derive(Debug, PartialEq)]
#[repr(u16)]
enum DataFrameType {
    Message,
    Padding,
}

#[derive(Debug)]
#[repr(C, packed)]
struct TransportHeader {
    protocol: TransportProtocol,
}

#[derive(Debug, PartialEq)]
#[repr(u16)]
enum TransportProtocol {
    RequestResponse,
    FullDuplexSingleMessage,
    ControlMessage,
}

#[derive(Debug, PartialEq)]
#[repr(u32)]
pub enum ControlMessageType {
    KeepAlive,
}

#[derive(Debug)]
#[repr(C, packed)]
struct RequestResponseHeader {
    request_id: u64,
}

#[derive(Debug)]
struct Decoder<'d> {
    data: &'d [u8],
    position: usize,
}

impl<'d> Decoder<'d> {
    fn new(data: &'d [u8]) -> Self {
        Decoder { data, position: 0 }
    }

    #[inline]
    fn read_type<T>(&mut self) -> Result<&'d T, Error> {
        let num_bytes = mem::size_of::<T>();
        let end = self.position + num_bytes;
        if end <= self.data.len() {
            let slice = self.data[self.position..end].as_ptr() as *mut T;
            let value: &'d T = unsafe { &*slice };
            self.position = end;
            Ok(value)
        } else {
            let available = self.data.len() - self.position;
            let error = format!(
                "Not enough bytes: only {} bytes left but {} required",
                available,
                num_bytes
            );
            Err(Error::new(ErrorKind::Other, error))
        }
    }

    #[inline]
    fn truncate(&mut self, length: usize) -> Result<(), Error> {
        if length < self.position {
            let error = format!(
                "Unable to truncate to {} before position {}",
                length,
                self.position
            );
            Err(Error::new(ErrorKind::Other, error))

        } else if length <= self.data.len() {
            self.data = &self.data[0..length];
            Ok(())
        } else {
            let error = format!(
                "Unable to truncate to {} with smaller length {}",
                length,
                self.data.len()
            );
            Err(Error::new(ErrorKind::Other, error))
        }

    }

    #[inline]
    fn remaining(self) -> &'d [u8] {
        &self.data[self.position..]
    }
}


#[derive(Debug)]
struct Encoder<'d> {
    data: &'d mut [u8],
    position: usize,
}

impl<'d> Encoder<'d> {
    fn new(data: &'d mut [u8]) -> Self {
        Encoder { data, position: 0 }
    }

    #[inline]
    fn write_type<T>(&mut self, value: &T) -> Result<(), Error> {
        let num_bytes = mem::size_of::<T>();
        let end = self.position + num_bytes;
        if end <= self.data.len() {
            let source_bytes: &[u8] = unsafe { slice::from_raw_parts(value as *const T as *const u8, num_bytes) };
            (&mut self.data[self.position..end]).copy_from_slice(source_bytes);
            self.position = end;
            Ok(())
        } else {
            let available = self.data.len() - self.position;
            let error = format!(
                "Not enough bytes: only {} bytes left but {} required",
                available,
                num_bytes
            );
            Err(Error::new(ErrorKind::Other, error))
        }

    }

    #[inline]
    fn write(&mut self, data: &[u8]) -> Result<(), Error> {
        let end = self.position + data.len();
        if end <= self.data.len() {
            (self.data[self.position..end]).copy_from_slice(data);
            Ok(())
        } else {
            let available = self.data.len() - self.position;
            let error = format!(
                "Not enough bytes: only {} bytes left but {} required",
                available,
                data.len()
            );
            Err(Error::new(ErrorKind::Other, error))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn decode_request_response() {
        let buffer = vec![
            // length
            27, 0, 0, 0,
            // version
            1,
            // flags
            12,
            // frame type
            0, 0,
            // stream id
            1, 4, 0, 0,
            // protocol,
            0, 0,
            // request id
            3, 2, 1, 0, 0, 0, 0, 0,
            // message
            1, 2, 3, 4, 5,
            // padding
            0, 0, 0, 0, 0,
        ];

        let frame = Frame::decode(&buffer).unwrap();

        if let Frame::RequestResponse(frame) = frame {
            let data_frame_header = frame.data_frame_header;
            assert_eq!(27, data_frame_header.length);
            assert_eq!(32, data_frame_header.aligned_length());
            assert_eq!(1, data_frame_header.version);
            assert_eq!(12, data_frame_header.flags);
            assert_eq!(DataFrameType::Message, data_frame_header.frame_type);
            assert_eq!(1025, data_frame_header.stream_id);

            let transport_header = frame.transport_header;
            assert_eq!(TransportProtocol::RequestResponse, transport_header.protocol);

            let request_response_header = frame.request_response_header;
            assert_eq!(66051, request_response_header.request_id);

            assert_eq!(vec![1, 2, 3, 4, 5], frame.message);

        } else {
            panic!("Expected request response frame but got {:?}", frame);
        }
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn encode_request_response() {
        let expected = vec![
            // length
            27, 0, 0, 0,
            // version
            1,
            // flags
            12,
            // frame type
            0, 0,
            // stream id
            1, 4, 0, 0,
            // protocol,
            0, 0,
            // request id
            3, 2, 1, 0, 0, 0, 0, 0,
            // message
            1, 2, 3, 4, 5,
            // padding
            0, 0, 0, 0, 0,
        ];

        let frame = request_response(1, 12, 1025, 66051, &vec![1, 2, 3, 4, 5]).unwrap();

        assert_eq!(expected, frame);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn decode_full_duplex_single_message() {
        let buffer = vec![
            // length
            19, 0, 0, 0,
            // version
            1,
            // flags
            12,
            // frame type
            0, 0,
            // stream id
            1, 4, 0, 0,
            // protocol,
            1, 0,
            // message
            1, 2, 3, 4, 5,
            // padding
            0, 0, 0, 0, 0
        ];

        let frame = Frame::decode(&buffer).unwrap();

        if let Frame::FullDuplexSingleMessage(frame) = frame {
            let data_frame_header = frame.data_frame_header;
            assert_eq!(19, data_frame_header.length);
            assert_eq!(24, data_frame_header.aligned_length());
            assert_eq!(1, data_frame_header.version);
            assert_eq!(12, data_frame_header.flags);
            assert_eq!(DataFrameType::Message, data_frame_header.frame_type);
            assert_eq!(1025, data_frame_header.stream_id);

            let transport_header = frame.transport_header;
            assert_eq!(TransportProtocol::FullDuplexSingleMessage, transport_header.protocol);

            assert_eq!(vec![1, 2, 3, 4, 5], frame.message);

        } else {
            panic!("Expected full duplex single message frame but got {:?}", frame);
        }
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn encode_full_duplex_single_message() {
        let expected = vec![
            // length
            19, 0, 0, 0,
            // version
            1,
            // flags
            12,
            // frame type
            0, 0,
            // stream id
            1, 4, 0, 0,
            // protocol,
            1, 0,
            // message
            1, 2, 3, 4, 5,
            // padding
            0, 0, 0, 0, 0,
        ];

        let frame = full_duplex_single_message(1, 12, 1025, &vec![1, 2, 3, 4, 5]).unwrap();

        assert_eq!(expected, frame);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn decode_keep_alive_message() {
        let buffer = vec![
            // length
            18, 0, 0, 0,
            // version
            1,
            // flags
            12,
            // frame type
            0, 0,
            // stream id
            1, 4, 0, 0,
            // protocol
            2, 0,
            // message type
            0, 0, 0, 0,
            // padding
            0, 0, 0, 0, 0, 0,
        ];

        let frame = Frame::decode(&buffer).unwrap();

        if let Frame::ControlMessage(frame) = frame {
            let data_frame_header = frame.data_frame_header;
            assert_eq!(18, data_frame_header.length);
            assert_eq!(24, data_frame_header.aligned_length());
            assert_eq!(1, data_frame_header.version);
            assert_eq!(12, data_frame_header.flags);
            assert_eq!(DataFrameType::Message, data_frame_header.frame_type);
            assert_eq!(1025, data_frame_header.stream_id);

            let transport_header = frame.transport_header;
            assert_eq!(TransportProtocol::ControlMessage, transport_header.protocol);

            assert_eq!(&ControlMessageType::KeepAlive, frame.message_type);
        } else {
            panic!("Expected control message frame but got {:?}", frame);
        }

    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn encode_keep_alive() {
        let expected = vec![
            // length
            18, 0, 0, 0,
            // version
            1,
            // flags
            12,
            // frame type
            0, 0,
            // stream id
            1, 4, 0, 0,
            // protocol
            2, 0,
            // message type
            0, 0, 0, 0,
            // padding
            0, 0, 0, 0, 0, 0,
        ];

        let frame = control_message(1, 12, 1025, &ControlMessageType::KeepAlive).unwrap();

        assert_eq!(expected, frame);
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn decode_padding() {
        let buffer = vec![
            // length
            13, 0, 0, 0,
            // version
            1,
            // flags
            12,
            // frame type
            1, 0,
            // stream id
            1, 4, 0, 0,
            // padding body
            0,
            // padding
            0, 0, 0,
        ];

        let frame = Frame::decode(&buffer).unwrap();

        if let Frame::Padding(frame) = frame {
            let data_frame_header = frame.data_frame_header;
            assert_eq!(13, data_frame_header.length);
            assert_eq!(16, data_frame_header.aligned_length());
            assert_eq!(1, data_frame_header.version);
            assert_eq!(12, data_frame_header.flags);
            assert_eq!(DataFrameType::Padding, data_frame_header.frame_type);
            assert_eq!(1025, data_frame_header.stream_id);

        } else {
            panic!("Expected padding frame but got {:?}", frame);
        }
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn encode_padding() {
        let expected = vec![
            // length
            13, 0, 0, 0,
            // version
            1,
            // flags
            12,
            // frame type
            1, 0,
            // stream id
            1, 4, 0, 0,
            // padding body
            0,
            // padding
            0, 0, 0,
        ];

        let frame = padding(1, 12, 1025, 1).unwrap();

        assert_eq!(expected, frame);
    }

}
