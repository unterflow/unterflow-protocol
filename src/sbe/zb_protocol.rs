#![allow(dead_code, non_camel_case_types, unused_must_use, unknown_lints, clippy)]

/// Generated code for SBE package zb_protocol

/// Imports core rather than std to broaden usable environments.
extern crate core;

/// Result types for error handling

/// Errors that may occur during the course of encoding or decoding.
#[derive(Debug)]
pub enum CodecErr {
    /// Too few bytes in the byte-slice to read or write the data structure relevant
    /// to the current state of the codec
    NotEnoughBytes,

    /// Groups and vardata are constrained by the numeric type chosen to represent their
    /// length as well as optional maxima imposed by the schema
    SliceIsLongerThanAllowedBySchema,
}

pub type CodecResult<T> = core::result::Result<T, CodecErr>;

/// Scratch Decoder Data Wrapper - codec internal use only
#[derive(Debug)]
struct ScratchDecoderData<'d> {
    data: &'d [u8],
    pos: usize,
}

impl<'d> ScratchDecoderData<'d> {
    /// Create a struct reference overlaid atop the data buffer
    /// such that the struct's contents directly reflect the buffer.
    /// Advances the `pos` index by the size of the struct in bytes.
    #[inline]
    fn read_type<T>(&mut self, num_bytes: usize) -> CodecResult<&'d T> {
        let end = self.pos + num_bytes;
        if end <= self.data.len() {
            let s = self.data[self.pos..end].as_ptr() as *mut T;
            let v: &'d T = unsafe { &*s };
            self.pos = end;
            Ok(v)
        } else {
            Err(CodecErr::NotEnoughBytes)
        }
    }

    /// Advances the `pos` index by a set number of bytes.
    #[inline]
    fn skip_bytes(&mut self, num_bytes: usize) -> CodecResult<()> {
        let end = self.pos + num_bytes;
        if end <= self.data.len() {
            self.pos = end;
            Ok(())
        } else {
            Err(CodecErr::NotEnoughBytes)
        }
    }

    /// Create a slice reference overlaid atop the data buffer
    /// such that the slice's members' contents directly reflect the buffer.
    /// Advances the `pos` index by the size of the slice contents in bytes.
    #[inline]
    fn read_slice<T>(&mut self, count: usize, bytes_per_item: usize) -> CodecResult<&'d [T]> {
        let num_bytes = bytes_per_item * count;
        let end = self.pos + num_bytes;
        if end <= self.data.len() {
            let v: &'d [T] = unsafe { core::slice::from_raw_parts(self.data[self.pos..end].as_ptr() as *const T, count) };
            self.pos = end;
            Ok(v)
        } else {
            Err(CodecErr::NotEnoughBytes)
        }
    }
}

/// Scratch Encoder Data Wrapper - codec internal use only
#[derive(Debug)]
struct ScratchEncoderData<'d> {
    data: &'d mut [u8],
    pos: usize,
}

impl<'d> ScratchEncoderData<'d> {
    /// Copy the bytes of a value into the data buffer
    /// Advances the `pos` index to after the newly-written bytes.
    #[inline]
    fn write_type<T>(&mut self, t: &T, num_bytes: usize) -> CodecResult<()> {
        let end = self.pos + num_bytes;
        if end <= self.data.len() {
            let source_bytes: &[u8] = unsafe { core::slice::from_raw_parts(t as *const T as *const u8, num_bytes) };
            (&mut self.data[self.pos..end]).copy_from_slice(source_bytes);
            self.pos = end;
            Ok(())
        } else {
            Err(CodecErr::NotEnoughBytes)
        }
    }

    /// Create a struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    /// Advances the `pos` index to after the newly-written bytes.
    #[inline]
    fn writable_overlay<T>(&mut self, num_bytes: usize) -> CodecResult<&'d mut T> {
        let end = self.pos + num_bytes;
        if end <= self.data.len() {
            let v: &'d mut T = unsafe {
                let s = self.data.as_ptr().offset(self.pos as isize) as *mut T;
                &mut *s
            };
            self.pos = end;
            Ok(v)
        } else {
            Err(CodecErr::NotEnoughBytes)
        }
    }

    /// Copy the bytes of a value into the data buffer at a specific position
    /// Does **not** alter the `pos` index.
    #[inline]
    fn write_at_position<T>(&mut self, position: usize, t: &T, num_bytes: usize) -> CodecResult<()> {
        let end = position + num_bytes;
        if end <= self.data.len() {
            let source_bytes: &[u8] = unsafe { core::slice::from_raw_parts(t as *const T as *const u8, num_bytes) };
            (&mut self.data[position..end]).copy_from_slice(source_bytes);
            Ok(())
        } else {
            Err(CodecErr::NotEnoughBytes)
        }
    }
    /// Create a mutable slice overlaid atop the data buffer directly
    /// such that changes to the slice contents directly edit the buffer
    /// Note that the initial content of the slice's members' fields may be garbage.
    /// Advances the `pos` index to after the region representing the slice.
    #[inline]
    fn writable_slice<T>(&mut self, count: usize, bytes_per_item: usize) -> CodecResult<&'d mut [T]> {
        let end = self.pos + (count * bytes_per_item);
        if end <= self.data.len() {
            let v: &'d mut [T] = unsafe { core::slice::from_raw_parts_mut(self.data[self.pos..end].as_mut_ptr() as *mut T, count) };
            self.pos = end;
            Ok(v)
        } else {
            Err(CodecErr::NotEnoughBytes)
        }
    }

    /// Copy the raw bytes of a slice's contents into the data buffer
    /// Does **not** encode the length of the slice explicitly into the buffer.
    /// Advances the `pos` index to after the newly-written slice bytes.
    #[inline]
    fn write_slice_without_count<T>(&mut self, t: &[T], bytes_per_item: usize) -> CodecResult<()> {
        let content_bytes_size = bytes_per_item * t.len();
        let end = self.pos + content_bytes_size;
        if end <= self.data.len() {
            let source_bytes: &[u8] = unsafe { core::slice::from_raw_parts(t.as_ptr() as *const u8, content_bytes_size) };
            (&mut self.data[self.pos..end]).copy_from_slice(source_bytes);
            self.pos = end;
            Ok(())
        } else {
            Err(CodecErr::NotEnoughBytes)
        }
    }
}

/// Convenience Either enum
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// Enum SubscriptionType
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SubscriptionType {
    TASK_SUBSCRIPTION = 0u8,
    TOPIC_SUBSCRIPTION = 1u8,
}

/// Enum EventType
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum EventType {
    TASK_EVENT = 0u8,
    RAFT_EVENT = 1u8,
    SUBSCRIPTION_EVENT = 2u8,
    SUBSCRIBER_EVENT = 3u8,
    DEPLOYMENT_EVENT = 4u8,
    WORKFLOW_INSTANCE_EVENT = 5u8,
    INCIDENT_EVENT = 6u8,
    WORKFLOW_EVENT = 7u8,
    NOOP_EVENT = 8u8,
    TOPIC_EVENT = 9u8,
    PARTITION_EVENT = 10u8,
}

/// Enum ControlMessageType
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ControlMessageType {
    ADD_TASK_SUBSCRIPTION = 0u8,
    REMOVE_TASK_SUBSCRIPTION = 1u8,
    INCREASE_TASK_SUBSCRIPTION_CREDITS = 2u8,
    REMOVE_TOPIC_SUBSCRIPTION = 3u8,
    REQUEST_TOPOLOGY = 4u8,
}

/// Enum ErrorCode
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ErrorCode {
    MESSAGE_NOT_SUPPORTED = 0u8,
    PARTITION_NOT_FOUND = 1u8,
    REQUEST_WRITE_FAILURE = 2u8,
    INVALID_CLIENT_VERSION = 3u8,
    REQUEST_TIMEOUT = 4u8,
    REQUEST_PROCESSING_FAILURE = 5u8,
    INVALID_MESSAGE = 6u8,
}

/// MessageHeader
#[derive(Debug, PartialEq)]
#[repr(C, packed)]
pub struct MessageHeader {
    pub block_length: u16,
    pub template_id: u16,
    pub schema_id: u16,
    pub version: u16,
}

impl MessageHeader {}

/// VarDataEncoding
#[repr(C, packed)]
pub struct VarDataEncoding {
    pub length: u16,
    pub var_data: u8,
}

impl VarDataEncoding {}

/// ErrorResponse Fixed-size Fields
#[repr(C, packed)]
pub struct ErrorResponseFields {
    pub error_code: ErrorCode,
}

impl ErrorResponseFields {}

/// ErrorResponse specific Message Header
#[repr(C, packed)]
pub struct ErrorResponseMessageHeader {
    pub message_header: MessageHeader,
}
impl Default for ErrorResponseMessageHeader {
    fn default() -> ErrorResponseMessageHeader {
        ErrorResponseMessageHeader {
            message_header: MessageHeader {
                block_length: 1u16,
                template_id: 0u16,
                schema_id: 0u16,
                version: 1u16,
            },
        }
    }
}

/// Group fixed-field member representations

/// ErrorResponseDecoderDone
pub struct ErrorResponseDecoderDone<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ErrorResponseDecoderDone<'d> {
    /// Returns the number of bytes decoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchDecoderData<'d>) -> ErrorResponseDecoderDone<'d> {
        ErrorResponseDecoderDone { scratch: scratch }
    }
}

/// failedRequest variable-length data
pub struct ErrorResponseFailedRequestDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ErrorResponseFailedRequestDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> Self {
        ErrorResponseFailedRequestDecoder { scratch: scratch }
    }
    pub fn failed_request(mut self) -> CodecResult<(&'d [u8], ErrorResponseDecoderDone<'d>)> {
        let count = *self.scratch.read_type::<u16>(2)?;
        Ok((
            self.scratch.read_slice::<u8>(count as usize, 1)?,
            ErrorResponseDecoderDone::wrap(self.scratch),
        ))
    }
}

/// errorData variable-length data
pub struct ErrorResponseErrorDataDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ErrorResponseErrorDataDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> Self {
        ErrorResponseErrorDataDecoder { scratch: scratch }
    }
    pub fn error_data(mut self) -> CodecResult<(&'d [u8], ErrorResponseFailedRequestDecoder<'d>)> {
        let count = *self.scratch.read_type::<u16>(2)?;
        Ok((
            self.scratch.read_slice::<u8>(count as usize, 1)?,
            ErrorResponseFailedRequestDecoder::wrap(self.scratch),
        ))
    }
}

/// ErrorResponse Fixed fields Decoder
pub struct ErrorResponseFieldsDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ErrorResponseFieldsDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> ErrorResponseFieldsDecoder<'d> {
        ErrorResponseFieldsDecoder { scratch: scratch }
    }
    pub fn error_response_fields(mut self) -> CodecResult<(&'d ErrorResponseFields, ErrorResponseErrorDataDecoder<'d>)> {
        let v = self.scratch.read_type::<ErrorResponseFields>(1)?;
        Ok((v, ErrorResponseErrorDataDecoder::wrap(self.scratch)))
    }
}

/// ErrorResponseMessageHeaderDecoder
pub struct ErrorResponseMessageHeaderDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ErrorResponseMessageHeaderDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> ErrorResponseMessageHeaderDecoder<'d> {
        ErrorResponseMessageHeaderDecoder { scratch: scratch }
    }
    pub fn header(mut self) -> CodecResult<(&'d MessageHeader, ErrorResponseFieldsDecoder<'d>)> {
        let v = self.scratch.read_type::<MessageHeader>(8)?;
        Ok((v, ErrorResponseFieldsDecoder::wrap(self.scratch)))
    }
}

/// ErrorResponse Decoder entry point
pub fn start_decoding_error_response<'d>(data: &'d [u8]) -> ErrorResponseMessageHeaderDecoder<'d> {
    ErrorResponseMessageHeaderDecoder::wrap(ScratchDecoderData { data: data, pos: 0 })
}

/// ErrorResponseEncoderDone
pub struct ErrorResponseEncoderDone<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ErrorResponseEncoderDone<'d> {
    /// Returns the number of bytes encoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchEncoderData<'d>) -> ErrorResponseEncoderDone<'d> {
        ErrorResponseEncoderDone { scratch: scratch }
    }
}

/// failedRequest variable-length data
pub struct ErrorResponseFailedRequestEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ErrorResponseFailedRequestEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> Self {
        ErrorResponseFailedRequestEncoder { scratch: scratch }
    }
    pub fn failed_request(mut self, s: &'d [u8]) -> CodecResult<ErrorResponseEncoderDone<'d>> {
        let l = s.len();
        if l > 65534 {
            return Err(CodecErr::SliceIsLongerThanAllowedBySchema);
        }
        // Write data length
        self.scratch.write_type::<u16>(&(l as u16), 2); // group length
        self.scratch.write_slice_without_count::<u8>(s, 1)?;
        Ok(ErrorResponseEncoderDone::wrap(self.scratch))
    }
}

/// errorData variable-length data
pub struct ErrorResponseErrorDataEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ErrorResponseErrorDataEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> Self {
        ErrorResponseErrorDataEncoder { scratch: scratch }
    }
    pub fn error_data(mut self, s: &'d [u8]) -> CodecResult<ErrorResponseFailedRequestEncoder<'d>> {
        let l = s.len();
        if l > 65534 {
            return Err(CodecErr::SliceIsLongerThanAllowedBySchema);
        }
        // Write data length
        self.scratch.write_type::<u16>(&(l as u16), 2); // group length
        self.scratch.write_slice_without_count::<u8>(s, 1)?;
        Ok(ErrorResponseFailedRequestEncoder::wrap(self.scratch))
    }
}

/// ErrorResponse Fixed fields Encoder
pub struct ErrorResponseFieldsEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ErrorResponseFieldsEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> ErrorResponseFieldsEncoder<'d> {
        ErrorResponseFieldsEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn error_response_fields(mut self) -> CodecResult<(&'d mut ErrorResponseFields, ErrorResponseErrorDataEncoder<'d>)> {
        let v = self.scratch.writable_overlay::<ErrorResponseFields>(1)?;
        Ok((v, ErrorResponseErrorDataEncoder::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn error_response_fields_copy(mut self, t: &ErrorResponseFields) -> CodecResult<ErrorResponseErrorDataEncoder<'d>> {
        self.scratch.write_type::<ErrorResponseFields>(t, 1)?;
        Ok(ErrorResponseErrorDataEncoder::wrap(self.scratch))
    }
}

/// ErrorResponseMessageHeaderEncoder
pub struct ErrorResponseMessageHeaderEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ErrorResponseMessageHeaderEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> ErrorResponseMessageHeaderEncoder<'d> {
        ErrorResponseMessageHeaderEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn header(mut self) -> CodecResult<(&'d mut MessageHeader, ErrorResponseFieldsEncoder<'d>)> {
        let v = self.scratch.writable_overlay::<MessageHeader>(8)?;
        Ok((v, ErrorResponseFieldsEncoder::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn header_copy(mut self, t: &MessageHeader) -> CodecResult<ErrorResponseFieldsEncoder<'d>> {
        self.scratch.write_type::<MessageHeader>(t, 8)?;
        Ok(ErrorResponseFieldsEncoder::wrap(self.scratch))
    }
}

/// ErrorResponse Encoder entry point
pub fn start_encoding_error_response<'d>(data: &'d mut [u8]) -> ErrorResponseMessageHeaderEncoder<'d> {
    ErrorResponseMessageHeaderEncoder::wrap(ScratchEncoderData { data: data, pos: 0 })
}

/// ExecuteCommandRequest Fixed-size Fields
#[repr(C, packed)]
pub struct ExecuteCommandRequestFields {
    pub partition_id: u16,
    pub position: u64,
    pub key: u64,
    pub event_type: EventType,
}

impl ExecuteCommandRequestFields {}

/// ExecuteCommandRequest specific Message Header
#[repr(C, packed)]
pub struct ExecuteCommandRequestMessageHeader {
    pub message_header: MessageHeader,
}
impl Default for ExecuteCommandRequestMessageHeader {
    fn default() -> ExecuteCommandRequestMessageHeader {
        ExecuteCommandRequestMessageHeader {
            message_header: MessageHeader {
                block_length: 19u16,
                template_id: 20u16,
                schema_id: 0u16,
                version: 1u16,
            },
        }
    }
}

/// Group fixed-field member representations

/// ExecuteCommandRequestDecoderDone
pub struct ExecuteCommandRequestDecoderDone<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ExecuteCommandRequestDecoderDone<'d> {
    /// Returns the number of bytes decoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchDecoderData<'d>) -> ExecuteCommandRequestDecoderDone<'d> {
        ExecuteCommandRequestDecoderDone { scratch: scratch }
    }
}

/// command variable-length data
pub struct ExecuteCommandRequestCommandDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ExecuteCommandRequestCommandDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> Self {
        ExecuteCommandRequestCommandDecoder { scratch: scratch }
    }
    pub fn command(mut self) -> CodecResult<(&'d [u8], ExecuteCommandRequestDecoderDone<'d>)> {
        let count = *self.scratch.read_type::<u16>(2)?;
        Ok((
            self.scratch.read_slice::<u8>(count as usize, 1)?,
            ExecuteCommandRequestDecoderDone::wrap(self.scratch),
        ))
    }
}

/// ExecuteCommandRequest Fixed fields Decoder
pub struct ExecuteCommandRequestFieldsDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ExecuteCommandRequestFieldsDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> ExecuteCommandRequestFieldsDecoder<'d> {
        ExecuteCommandRequestFieldsDecoder { scratch: scratch }
    }
    pub fn execute_command_request_fields(mut self) -> CodecResult<(&'d ExecuteCommandRequestFields, ExecuteCommandRequestCommandDecoder<'d>)> {
        let v = self.scratch.read_type::<ExecuteCommandRequestFields>(19)?;
        Ok((v, ExecuteCommandRequestCommandDecoder::wrap(self.scratch)))
    }
}

/// ExecuteCommandRequestMessageHeaderDecoder
pub struct ExecuteCommandRequestMessageHeaderDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ExecuteCommandRequestMessageHeaderDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> ExecuteCommandRequestMessageHeaderDecoder<'d> {
        ExecuteCommandRequestMessageHeaderDecoder { scratch: scratch }
    }
    pub fn header(mut self) -> CodecResult<(&'d MessageHeader, ExecuteCommandRequestFieldsDecoder<'d>)> {
        let v = self.scratch.read_type::<MessageHeader>(8)?;
        Ok((v, ExecuteCommandRequestFieldsDecoder::wrap(self.scratch)))
    }
}

/// ExecuteCommandRequest Decoder entry point
pub fn start_decoding_execute_command_request<'d>(data: &'d [u8]) -> ExecuteCommandRequestMessageHeaderDecoder<'d> {
    ExecuteCommandRequestMessageHeaderDecoder::wrap(ScratchDecoderData { data: data, pos: 0 })
}

/// ExecuteCommandRequestEncoderDone
pub struct ExecuteCommandRequestEncoderDone<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ExecuteCommandRequestEncoderDone<'d> {
    /// Returns the number of bytes encoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchEncoderData<'d>) -> ExecuteCommandRequestEncoderDone<'d> {
        ExecuteCommandRequestEncoderDone { scratch: scratch }
    }
}

/// command variable-length data
pub struct ExecuteCommandRequestCommandEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ExecuteCommandRequestCommandEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> Self {
        ExecuteCommandRequestCommandEncoder { scratch: scratch }
    }
    pub fn command(mut self, s: &'d [u8]) -> CodecResult<ExecuteCommandRequestEncoderDone<'d>> {
        let l = s.len();
        if l > 65534 {
            return Err(CodecErr::SliceIsLongerThanAllowedBySchema);
        }
        // Write data length
        self.scratch.write_type::<u16>(&(l as u16), 2); // group length
        self.scratch.write_slice_without_count::<u8>(s, 1)?;
        Ok(ExecuteCommandRequestEncoderDone::wrap(self.scratch))
    }
}

/// ExecuteCommandRequest Fixed fields Encoder
pub struct ExecuteCommandRequestFieldsEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ExecuteCommandRequestFieldsEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> ExecuteCommandRequestFieldsEncoder<'d> {
        ExecuteCommandRequestFieldsEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn execute_command_request_fields(mut self) -> CodecResult<(&'d mut ExecuteCommandRequestFields, ExecuteCommandRequestCommandEncoder<'d>)> {
        let v = self.scratch
            .writable_overlay::<ExecuteCommandRequestFields>(19)?;
        Ok((v, ExecuteCommandRequestCommandEncoder::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn execute_command_request_fields_copy(mut self, t: &ExecuteCommandRequestFields) -> CodecResult<ExecuteCommandRequestCommandEncoder<'d>> {
        self.scratch.write_type::<ExecuteCommandRequestFields>(
            t,
            19,
        )?;
        Ok(ExecuteCommandRequestCommandEncoder::wrap(self.scratch))
    }
}

/// ExecuteCommandRequestMessageHeaderEncoder
pub struct ExecuteCommandRequestMessageHeaderEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ExecuteCommandRequestMessageHeaderEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> ExecuteCommandRequestMessageHeaderEncoder<'d> {
        ExecuteCommandRequestMessageHeaderEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn header(mut self) -> CodecResult<(&'d mut MessageHeader, ExecuteCommandRequestFieldsEncoder<'d>)> {
        let v = self.scratch.writable_overlay::<MessageHeader>(8)?;
        Ok((v, ExecuteCommandRequestFieldsEncoder::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn header_copy(mut self, t: &MessageHeader) -> CodecResult<ExecuteCommandRequestFieldsEncoder<'d>> {
        self.scratch.write_type::<MessageHeader>(t, 8)?;
        Ok(ExecuteCommandRequestFieldsEncoder::wrap(self.scratch))
    }
}

/// ExecuteCommandRequest Encoder entry point
pub fn start_encoding_execute_command_request<'d>(data: &'d mut [u8]) -> ExecuteCommandRequestMessageHeaderEncoder<'d> {
    ExecuteCommandRequestMessageHeaderEncoder::wrap(ScratchEncoderData { data: data, pos: 0 })
}

/// ExecuteCommandResponse Fixed-size Fields
#[repr(C, packed)]
pub struct ExecuteCommandResponseFields {
    pub partition_id: u16,
    pub position: u64,
    pub key: u64,
}

impl ExecuteCommandResponseFields {}

/// ExecuteCommandResponse specific Message Header
#[repr(C, packed)]
pub struct ExecuteCommandResponseMessageHeader {
    pub message_header: MessageHeader,
}
impl Default for ExecuteCommandResponseMessageHeader {
    fn default() -> ExecuteCommandResponseMessageHeader {
        ExecuteCommandResponseMessageHeader {
            message_header: MessageHeader {
                block_length: 18u16,
                template_id: 21u16,
                schema_id: 0u16,
                version: 1u16,
            },
        }
    }
}

/// Group fixed-field member representations

/// ExecuteCommandResponseDecoderDone
pub struct ExecuteCommandResponseDecoderDone<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ExecuteCommandResponseDecoderDone<'d> {
    /// Returns the number of bytes decoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchDecoderData<'d>) -> ExecuteCommandResponseDecoderDone<'d> {
        ExecuteCommandResponseDecoderDone { scratch: scratch }
    }
}

/// event variable-length data
pub struct ExecuteCommandResponseEventDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ExecuteCommandResponseEventDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> Self {
        ExecuteCommandResponseEventDecoder { scratch: scratch }
    }
    pub fn event(mut self) -> CodecResult<(&'d [u8], ExecuteCommandResponseDecoderDone<'d>)> {
        let count = *self.scratch.read_type::<u16>(2)?;
        Ok((
            self.scratch.read_slice::<u8>(count as usize, 1)?,
            ExecuteCommandResponseDecoderDone::wrap(self.scratch),
        ))
    }
}

/// ExecuteCommandResponse Fixed fields Decoder
pub struct ExecuteCommandResponseFieldsDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ExecuteCommandResponseFieldsDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> ExecuteCommandResponseFieldsDecoder<'d> {
        ExecuteCommandResponseFieldsDecoder { scratch: scratch }
    }
    pub fn execute_command_response_fields(mut self) -> CodecResult<(&'d ExecuteCommandResponseFields, ExecuteCommandResponseEventDecoder<'d>)> {
        let v = self.scratch.read_type::<ExecuteCommandResponseFields>(18)?;
        Ok((v, ExecuteCommandResponseEventDecoder::wrap(self.scratch)))
    }
}

/// ExecuteCommandResponseMessageHeaderDecoder
pub struct ExecuteCommandResponseMessageHeaderDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ExecuteCommandResponseMessageHeaderDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> ExecuteCommandResponseMessageHeaderDecoder<'d> {
        ExecuteCommandResponseMessageHeaderDecoder { scratch: scratch }
    }
    pub fn header(mut self) -> CodecResult<(&'d MessageHeader, ExecuteCommandResponseFieldsDecoder<'d>)> {
        let v = self.scratch.read_type::<MessageHeader>(8)?;
        Ok((v, ExecuteCommandResponseFieldsDecoder::wrap(self.scratch)))
    }
}

/// ExecuteCommandResponse Decoder entry point
pub fn start_decoding_execute_command_response<'d>(data: &'d [u8]) -> ExecuteCommandResponseMessageHeaderDecoder<'d> {
    ExecuteCommandResponseMessageHeaderDecoder::wrap(ScratchDecoderData { data: data, pos: 0 })
}

/// ExecuteCommandResponseEncoderDone
pub struct ExecuteCommandResponseEncoderDone<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ExecuteCommandResponseEncoderDone<'d> {
    /// Returns the number of bytes encoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchEncoderData<'d>) -> ExecuteCommandResponseEncoderDone<'d> {
        ExecuteCommandResponseEncoderDone { scratch: scratch }
    }
}

/// event variable-length data
pub struct ExecuteCommandResponseEventEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ExecuteCommandResponseEventEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> Self {
        ExecuteCommandResponseEventEncoder { scratch: scratch }
    }
    pub fn event(mut self, s: &'d [u8]) -> CodecResult<ExecuteCommandResponseEncoderDone<'d>> {
        let l = s.len();
        if l > 65534 {
            return Err(CodecErr::SliceIsLongerThanAllowedBySchema);
        }
        // Write data length
        self.scratch.write_type::<u16>(&(l as u16), 2); // group length
        self.scratch.write_slice_without_count::<u8>(s, 1)?;
        Ok(ExecuteCommandResponseEncoderDone::wrap(self.scratch))
    }
}

/// ExecuteCommandResponse Fixed fields Encoder
pub struct ExecuteCommandResponseFieldsEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ExecuteCommandResponseFieldsEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> ExecuteCommandResponseFieldsEncoder<'d> {
        ExecuteCommandResponseFieldsEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn execute_command_response_fields(mut self) -> CodecResult<(&'d mut ExecuteCommandResponseFields, ExecuteCommandResponseEventEncoder<'d>)> {
        let v = self.scratch
            .writable_overlay::<ExecuteCommandResponseFields>(18)?;
        Ok((v, ExecuteCommandResponseEventEncoder::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn execute_command_response_fields_copy(mut self, t: &ExecuteCommandResponseFields) -> CodecResult<ExecuteCommandResponseEventEncoder<'d>> {
        self.scratch.write_type::<ExecuteCommandResponseFields>(
            t,
            18,
        )?;
        Ok(ExecuteCommandResponseEventEncoder::wrap(self.scratch))
    }
}

/// ExecuteCommandResponseMessageHeaderEncoder
pub struct ExecuteCommandResponseMessageHeaderEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ExecuteCommandResponseMessageHeaderEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> ExecuteCommandResponseMessageHeaderEncoder<'d> {
        ExecuteCommandResponseMessageHeaderEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn header(mut self) -> CodecResult<(&'d mut MessageHeader, ExecuteCommandResponseFieldsEncoder<'d>)> {
        let v = self.scratch.writable_overlay::<MessageHeader>(8)?;
        Ok((v, ExecuteCommandResponseFieldsEncoder::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn header_copy(mut self, t: &MessageHeader) -> CodecResult<ExecuteCommandResponseFieldsEncoder<'d>> {
        self.scratch.write_type::<MessageHeader>(t, 8)?;
        Ok(ExecuteCommandResponseFieldsEncoder::wrap(self.scratch))
    }
}

/// ExecuteCommandResponse Encoder entry point
pub fn start_encoding_execute_command_response<'d>(data: &'d mut [u8]) -> ExecuteCommandResponseMessageHeaderEncoder<'d> {
    ExecuteCommandResponseMessageHeaderEncoder::wrap(ScratchEncoderData { data: data, pos: 0 })
}

/// BrokerEventMetadata Fixed-size Fields
#[repr(C, packed)]
pub struct BrokerEventMetadataFields {
    pub request_stream_id: i32,
    pub request_id: u64,
    pub subscription_id: u64,
    pub protocol_version: u16,
    pub event_type: EventType,
    pub incident_key: u64,
}

impl BrokerEventMetadataFields {}

/// BrokerEventMetadata specific Message Header
#[repr(C, packed)]
pub struct BrokerEventMetadataMessageHeader {
    pub message_header: MessageHeader,
}
impl Default for BrokerEventMetadataMessageHeader {
    fn default() -> BrokerEventMetadataMessageHeader {
        BrokerEventMetadataMessageHeader {
            message_header: MessageHeader {
                block_length: 31u16,
                template_id: 200u16,
                schema_id: 0u16,
                version: 1u16,
            },
        }
    }
}

/// Group fixed-field member representations

/// BrokerEventMetadataDecoderDone
pub struct BrokerEventMetadataDecoderDone<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> BrokerEventMetadataDecoderDone<'d> {
    /// Returns the number of bytes decoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchDecoderData<'d>) -> BrokerEventMetadataDecoderDone<'d> {
        BrokerEventMetadataDecoderDone { scratch: scratch }
    }
}

/// BrokerEventMetadata Fixed fields Decoder
pub struct BrokerEventMetadataFieldsDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> BrokerEventMetadataFieldsDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> BrokerEventMetadataFieldsDecoder<'d> {
        BrokerEventMetadataFieldsDecoder { scratch: scratch }
    }
    pub fn broker_event_metadata_fields(mut self) -> CodecResult<(&'d BrokerEventMetadataFields, BrokerEventMetadataDecoderDone<'d>)> {
        let v = self.scratch.read_type::<BrokerEventMetadataFields>(31)?;
        Ok((v, BrokerEventMetadataDecoderDone::wrap(self.scratch)))
    }
}

/// BrokerEventMetadataMessageHeaderDecoder
pub struct BrokerEventMetadataMessageHeaderDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> BrokerEventMetadataMessageHeaderDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> BrokerEventMetadataMessageHeaderDecoder<'d> {
        BrokerEventMetadataMessageHeaderDecoder { scratch: scratch }
    }
    pub fn header(mut self) -> CodecResult<(&'d MessageHeader, BrokerEventMetadataFieldsDecoder<'d>)> {
        let v = self.scratch.read_type::<MessageHeader>(8)?;
        Ok((v, BrokerEventMetadataFieldsDecoder::wrap(self.scratch)))
    }
}

/// BrokerEventMetadata Decoder entry point
pub fn start_decoding_broker_event_metadata<'d>(data: &'d [u8]) -> BrokerEventMetadataMessageHeaderDecoder<'d> {
    BrokerEventMetadataMessageHeaderDecoder::wrap(ScratchDecoderData { data: data, pos: 0 })
}

/// BrokerEventMetadataEncoderDone
pub struct BrokerEventMetadataEncoderDone<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> BrokerEventMetadataEncoderDone<'d> {
    /// Returns the number of bytes encoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchEncoderData<'d>) -> BrokerEventMetadataEncoderDone<'d> {
        BrokerEventMetadataEncoderDone { scratch: scratch }
    }
}

/// BrokerEventMetadata Fixed fields Encoder
pub struct BrokerEventMetadataFieldsEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> BrokerEventMetadataFieldsEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> BrokerEventMetadataFieldsEncoder<'d> {
        BrokerEventMetadataFieldsEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn broker_event_metadata_fields(mut self) -> CodecResult<(&'d mut BrokerEventMetadataFields, BrokerEventMetadataEncoderDone<'d>)> {
        let v = self.scratch.writable_overlay::<BrokerEventMetadataFields>(
            31,
        )?;
        Ok((v, BrokerEventMetadataEncoderDone::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn broker_event_metadata_fields_copy(mut self, t: &BrokerEventMetadataFields) -> CodecResult<BrokerEventMetadataEncoderDone<'d>> {
        self.scratch.write_type::<BrokerEventMetadataFields>(t, 31)?;
        Ok(BrokerEventMetadataEncoderDone::wrap(self.scratch))
    }
}

/// BrokerEventMetadataMessageHeaderEncoder
pub struct BrokerEventMetadataMessageHeaderEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> BrokerEventMetadataMessageHeaderEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> BrokerEventMetadataMessageHeaderEncoder<'d> {
        BrokerEventMetadataMessageHeaderEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn header(mut self) -> CodecResult<(&'d mut MessageHeader, BrokerEventMetadataFieldsEncoder<'d>)> {
        let v = self.scratch.writable_overlay::<MessageHeader>(8)?;
        Ok((v, BrokerEventMetadataFieldsEncoder::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn header_copy(mut self, t: &MessageHeader) -> CodecResult<BrokerEventMetadataFieldsEncoder<'d>> {
        self.scratch.write_type::<MessageHeader>(t, 8)?;
        Ok(BrokerEventMetadataFieldsEncoder::wrap(self.scratch))
    }
}

/// BrokerEventMetadata Encoder entry point
pub fn start_encoding_broker_event_metadata<'d>(data: &'d mut [u8]) -> BrokerEventMetadataMessageHeaderEncoder<'d> {
    BrokerEventMetadataMessageHeaderEncoder::wrap(ScratchEncoderData { data: data, pos: 0 })
}

/// ControlMessageRequest Fixed-size Fields
#[repr(C, packed)]
pub struct ControlMessageRequestFields {
    pub message_type: ControlMessageType,
    pub partition_id: u16,
}

impl ControlMessageRequestFields {}

/// ControlMessageRequest specific Message Header
#[repr(C, packed)]
pub struct ControlMessageRequestMessageHeader {
    pub message_header: MessageHeader,
}
impl Default for ControlMessageRequestMessageHeader {
    fn default() -> ControlMessageRequestMessageHeader {
        ControlMessageRequestMessageHeader {
            message_header: MessageHeader {
                block_length: 3u16,
                template_id: 10u16,
                schema_id: 0u16,
                version: 1u16,
            },
        }
    }
}

/// Group fixed-field member representations

/// ControlMessageRequestDecoderDone
pub struct ControlMessageRequestDecoderDone<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ControlMessageRequestDecoderDone<'d> {
    /// Returns the number of bytes decoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchDecoderData<'d>) -> ControlMessageRequestDecoderDone<'d> {
        ControlMessageRequestDecoderDone { scratch: scratch }
    }
}

/// data variable-length data
pub struct ControlMessageRequestDataDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ControlMessageRequestDataDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> Self {
        ControlMessageRequestDataDecoder { scratch: scratch }
    }
    pub fn data(mut self) -> CodecResult<(&'d [u8], ControlMessageRequestDecoderDone<'d>)> {
        let count = *self.scratch.read_type::<u16>(2)?;
        Ok((
            self.scratch.read_slice::<u8>(count as usize, 1)?,
            ControlMessageRequestDecoderDone::wrap(self.scratch),
        ))
    }
}

/// ControlMessageRequest Fixed fields Decoder
pub struct ControlMessageRequestFieldsDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ControlMessageRequestFieldsDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> ControlMessageRequestFieldsDecoder<'d> {
        ControlMessageRequestFieldsDecoder { scratch: scratch }
    }
    pub fn control_message_request_fields(mut self) -> CodecResult<(&'d ControlMessageRequestFields, ControlMessageRequestDataDecoder<'d>)> {
        let v = self.scratch.read_type::<ControlMessageRequestFields>(3)?;
        Ok((v, ControlMessageRequestDataDecoder::wrap(self.scratch)))
    }
}

/// ControlMessageRequestMessageHeaderDecoder
pub struct ControlMessageRequestMessageHeaderDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ControlMessageRequestMessageHeaderDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> ControlMessageRequestMessageHeaderDecoder<'d> {
        ControlMessageRequestMessageHeaderDecoder { scratch: scratch }
    }
    pub fn header(mut self) -> CodecResult<(&'d MessageHeader, ControlMessageRequestFieldsDecoder<'d>)> {
        let v = self.scratch.read_type::<MessageHeader>(8)?;
        Ok((v, ControlMessageRequestFieldsDecoder::wrap(self.scratch)))
    }
}

/// ControlMessageRequest Decoder entry point
pub fn start_decoding_control_message_request<'d>(data: &'d [u8]) -> ControlMessageRequestMessageHeaderDecoder<'d> {
    ControlMessageRequestMessageHeaderDecoder::wrap(ScratchDecoderData { data: data, pos: 0 })
}

/// ControlMessageRequestEncoderDone
pub struct ControlMessageRequestEncoderDone<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ControlMessageRequestEncoderDone<'d> {
    /// Returns the number of bytes encoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchEncoderData<'d>) -> ControlMessageRequestEncoderDone<'d> {
        ControlMessageRequestEncoderDone { scratch: scratch }
    }
}

/// data variable-length data
pub struct ControlMessageRequestDataEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ControlMessageRequestDataEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> Self {
        ControlMessageRequestDataEncoder { scratch: scratch }
    }
    pub fn data(mut self, s: &'d [u8]) -> CodecResult<ControlMessageRequestEncoderDone<'d>> {
        let l = s.len();
        if l > 65534 {
            return Err(CodecErr::SliceIsLongerThanAllowedBySchema);
        }
        // Write data length
        self.scratch.write_type::<u16>(&(l as u16), 2); // group length
        self.scratch.write_slice_without_count::<u8>(s, 1)?;
        Ok(ControlMessageRequestEncoderDone::wrap(self.scratch))
    }
}

/// ControlMessageRequest Fixed fields Encoder
pub struct ControlMessageRequestFieldsEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ControlMessageRequestFieldsEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> ControlMessageRequestFieldsEncoder<'d> {
        ControlMessageRequestFieldsEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn control_message_request_fields(mut self) -> CodecResult<(&'d mut ControlMessageRequestFields, ControlMessageRequestDataEncoder<'d>)> {
        let v = self.scratch
            .writable_overlay::<ControlMessageRequestFields>(3)?;
        Ok((v, ControlMessageRequestDataEncoder::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn control_message_request_fields_copy(mut self, t: &ControlMessageRequestFields) -> CodecResult<ControlMessageRequestDataEncoder<'d>> {
        self.scratch.write_type::<ControlMessageRequestFields>(t, 3)?;
        Ok(ControlMessageRequestDataEncoder::wrap(self.scratch))
    }
}

/// ControlMessageRequestMessageHeaderEncoder
pub struct ControlMessageRequestMessageHeaderEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ControlMessageRequestMessageHeaderEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> ControlMessageRequestMessageHeaderEncoder<'d> {
        ControlMessageRequestMessageHeaderEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn header(mut self) -> CodecResult<(&'d mut MessageHeader, ControlMessageRequestFieldsEncoder<'d>)> {
        let v = self.scratch.writable_overlay::<MessageHeader>(8)?;
        Ok((v, ControlMessageRequestFieldsEncoder::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn header_copy(mut self, t: &MessageHeader) -> CodecResult<ControlMessageRequestFieldsEncoder<'d>> {
        self.scratch.write_type::<MessageHeader>(t, 8)?;
        Ok(ControlMessageRequestFieldsEncoder::wrap(self.scratch))
    }
}

/// ControlMessageRequest Encoder entry point
pub fn start_encoding_control_message_request<'d>(data: &'d mut [u8]) -> ControlMessageRequestMessageHeaderEncoder<'d> {
    ControlMessageRequestMessageHeaderEncoder::wrap(ScratchEncoderData { data: data, pos: 0 })
}

/// ControlMessageResponse specific Message Header
#[repr(C, packed)]
pub struct ControlMessageResponseMessageHeader {
    pub message_header: MessageHeader,
}
impl Default for ControlMessageResponseMessageHeader {
    fn default() -> ControlMessageResponseMessageHeader {
        ControlMessageResponseMessageHeader {
            message_header: MessageHeader {
                block_length: 0u16,
                template_id: 11u16,
                schema_id: 0u16,
                version: 1u16,
            },
        }
    }
}

/// Group fixed-field member representations

/// ControlMessageResponseDecoderDone
pub struct ControlMessageResponseDecoderDone<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ControlMessageResponseDecoderDone<'d> {
    /// Returns the number of bytes decoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchDecoderData<'d>) -> ControlMessageResponseDecoderDone<'d> {
        ControlMessageResponseDecoderDone { scratch: scratch }
    }
}

/// data variable-length data
pub struct ControlMessageResponseDataDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ControlMessageResponseDataDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> Self {
        ControlMessageResponseDataDecoder { scratch: scratch }
    }
    pub fn data(mut self) -> CodecResult<(&'d [u8], ControlMessageResponseDecoderDone<'d>)> {
        let count = *self.scratch.read_type::<u16>(2)?;
        Ok((
            self.scratch.read_slice::<u8>(count as usize, 1)?,
            ControlMessageResponseDecoderDone::wrap(self.scratch),
        ))
    }
}

/// ControlMessageResponseMessageHeaderDecoder
pub struct ControlMessageResponseMessageHeaderDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> ControlMessageResponseMessageHeaderDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> ControlMessageResponseMessageHeaderDecoder<'d> {
        ControlMessageResponseMessageHeaderDecoder { scratch: scratch }
    }
    pub fn header(mut self) -> CodecResult<(&'d MessageHeader, ControlMessageResponseDataDecoder<'d>)> {
        let v = self.scratch.read_type::<MessageHeader>(8)?;
        Ok((v, ControlMessageResponseDataDecoder::wrap(self.scratch)))
    }
}

/// ControlMessageResponse Decoder entry point
pub fn start_decoding_control_message_response<'d>(data: &'d [u8]) -> ControlMessageResponseMessageHeaderDecoder<'d> {
    ControlMessageResponseMessageHeaderDecoder::wrap(ScratchDecoderData { data: data, pos: 0 })
}

/// ControlMessageResponseEncoderDone
pub struct ControlMessageResponseEncoderDone<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ControlMessageResponseEncoderDone<'d> {
    /// Returns the number of bytes encoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchEncoderData<'d>) -> ControlMessageResponseEncoderDone<'d> {
        ControlMessageResponseEncoderDone { scratch: scratch }
    }
}

/// data variable-length data
pub struct ControlMessageResponseDataEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ControlMessageResponseDataEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> Self {
        ControlMessageResponseDataEncoder { scratch: scratch }
    }
    pub fn data(mut self, s: &'d [u8]) -> CodecResult<ControlMessageResponseEncoderDone<'d>> {
        let l = s.len();
        if l > 65534 {
            return Err(CodecErr::SliceIsLongerThanAllowedBySchema);
        }
        // Write data length
        self.scratch.write_type::<u16>(&(l as u16), 2); // group length
        self.scratch.write_slice_without_count::<u8>(s, 1)?;
        Ok(ControlMessageResponseEncoderDone::wrap(self.scratch))
    }
}

/// ControlMessageResponseMessageHeaderEncoder
pub struct ControlMessageResponseMessageHeaderEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> ControlMessageResponseMessageHeaderEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> ControlMessageResponseMessageHeaderEncoder<'d> {
        ControlMessageResponseMessageHeaderEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn header(mut self) -> CodecResult<(&'d mut MessageHeader, ControlMessageResponseDataEncoder<'d>)> {
        let v = self.scratch.writable_overlay::<MessageHeader>(8)?;
        Ok((v, ControlMessageResponseDataEncoder::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn header_copy(mut self, t: &MessageHeader) -> CodecResult<ControlMessageResponseDataEncoder<'d>> {
        self.scratch.write_type::<MessageHeader>(t, 8)?;
        Ok(ControlMessageResponseDataEncoder::wrap(self.scratch))
    }
}

/// ControlMessageResponse Encoder entry point
pub fn start_encoding_control_message_response<'d>(data: &'d mut [u8]) -> ControlMessageResponseMessageHeaderEncoder<'d> {
    ControlMessageResponseMessageHeaderEncoder::wrap(ScratchEncoderData { data: data, pos: 0 })
}

/// SubscribedEvent Fixed-size Fields
#[repr(C, packed)]
pub struct SubscribedEventFields {
    pub partition_id: u16,
    pub position: u64,
    pub key: u64,
    pub subscriber_key: u64,
    pub subscription_type: SubscriptionType,
    pub event_type: EventType,
}

impl SubscribedEventFields {}

/// SubscribedEvent specific Message Header
#[repr(C, packed)]
pub struct SubscribedEventMessageHeader {
    pub message_header: MessageHeader,
}
impl Default for SubscribedEventMessageHeader {
    fn default() -> SubscribedEventMessageHeader {
        SubscribedEventMessageHeader {
            message_header: MessageHeader {
                block_length: 28u16,
                template_id: 30u16,
                schema_id: 0u16,
                version: 1u16,
            },
        }
    }
}

/// Group fixed-field member representations

/// SubscribedEventDecoderDone
pub struct SubscribedEventDecoderDone<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> SubscribedEventDecoderDone<'d> {
    /// Returns the number of bytes decoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchDecoderData<'d>) -> SubscribedEventDecoderDone<'d> {
        SubscribedEventDecoderDone { scratch: scratch }
    }
}

/// event variable-length data
pub struct SubscribedEventEventDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> SubscribedEventEventDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> Self {
        SubscribedEventEventDecoder { scratch: scratch }
    }
    pub fn event(mut self) -> CodecResult<(&'d [u8], SubscribedEventDecoderDone<'d>)> {
        let count = *self.scratch.read_type::<u16>(2)?;
        Ok((
            self.scratch.read_slice::<u8>(count as usize, 1)?,
            SubscribedEventDecoderDone::wrap(self.scratch),
        ))
    }
}

/// SubscribedEvent Fixed fields Decoder
pub struct SubscribedEventFieldsDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> SubscribedEventFieldsDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> SubscribedEventFieldsDecoder<'d> {
        SubscribedEventFieldsDecoder { scratch: scratch }
    }
    pub fn subscribed_event_fields(mut self) -> CodecResult<(&'d SubscribedEventFields, SubscribedEventEventDecoder<'d>)> {
        let v = self.scratch.read_type::<SubscribedEventFields>(28)?;
        Ok((v, SubscribedEventEventDecoder::wrap(self.scratch)))
    }
}

/// SubscribedEventMessageHeaderDecoder
pub struct SubscribedEventMessageHeaderDecoder<'d> {
    scratch: ScratchDecoderData<'d>,
}
impl<'d> SubscribedEventMessageHeaderDecoder<'d> {
    fn wrap(scratch: ScratchDecoderData<'d>) -> SubscribedEventMessageHeaderDecoder<'d> {
        SubscribedEventMessageHeaderDecoder { scratch: scratch }
    }
    pub fn header(mut self) -> CodecResult<(&'d MessageHeader, SubscribedEventFieldsDecoder<'d>)> {
        let v = self.scratch.read_type::<MessageHeader>(8)?;
        Ok((v, SubscribedEventFieldsDecoder::wrap(self.scratch)))
    }
}

/// SubscribedEvent Decoder entry point
pub fn start_decoding_subscribed_event<'d>(data: &'d [u8]) -> SubscribedEventMessageHeaderDecoder<'d> {
    SubscribedEventMessageHeaderDecoder::wrap(ScratchDecoderData { data: data, pos: 0 })
}

/// SubscribedEventEncoderDone
pub struct SubscribedEventEncoderDone<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> SubscribedEventEncoderDone<'d> {
    /// Returns the number of bytes encoded
    pub fn unwrap(self) -> usize {
        self.scratch.pos
    }

    fn wrap(scratch: ScratchEncoderData<'d>) -> SubscribedEventEncoderDone<'d> {
        SubscribedEventEncoderDone { scratch: scratch }
    }
}

/// event variable-length data
pub struct SubscribedEventEventEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> SubscribedEventEventEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> Self {
        SubscribedEventEventEncoder { scratch: scratch }
    }
    pub fn event(mut self, s: &'d [u8]) -> CodecResult<SubscribedEventEncoderDone<'d>> {
        let l = s.len();
        if l > 65534 {
            return Err(CodecErr::SliceIsLongerThanAllowedBySchema);
        }
        // Write data length
        self.scratch.write_type::<u16>(&(l as u16), 2); // group length
        self.scratch.write_slice_without_count::<u8>(s, 1)?;
        Ok(SubscribedEventEncoderDone::wrap(self.scratch))
    }
}

/// SubscribedEvent Fixed fields Encoder
pub struct SubscribedEventFieldsEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> SubscribedEventFieldsEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> SubscribedEventFieldsEncoder<'d> {
        SubscribedEventFieldsEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn subscribed_event_fields(mut self) -> CodecResult<(&'d mut SubscribedEventFields, SubscribedEventEventEncoder<'d>)> {
        let v = self.scratch.writable_overlay::<SubscribedEventFields>(28)?;
        Ok((v, SubscribedEventEventEncoder::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn subscribed_event_fields_copy(mut self, t: &SubscribedEventFields) -> CodecResult<SubscribedEventEventEncoder<'d>> {
        self.scratch.write_type::<SubscribedEventFields>(t, 28)?;
        Ok(SubscribedEventEventEncoder::wrap(self.scratch))
    }
}

/// SubscribedEventMessageHeaderEncoder
pub struct SubscribedEventMessageHeaderEncoder<'d> {
    scratch: ScratchEncoderData<'d>,
}
impl<'d> SubscribedEventMessageHeaderEncoder<'d> {
    fn wrap(scratch: ScratchEncoderData<'d>) -> SubscribedEventMessageHeaderEncoder<'d> {
        SubscribedEventMessageHeaderEncoder { scratch: scratch }
    }

    /// Create a mutable struct reference overlaid atop the data buffer
    /// such that changes to the struct directly edit the buffer.
    /// Note that the initial content of the struct's fields may be garbage.
    pub fn header(mut self) -> CodecResult<(&'d mut MessageHeader, SubscribedEventFieldsEncoder<'d>)> {
        let v = self.scratch.writable_overlay::<MessageHeader>(8)?;
        Ok((v, SubscribedEventFieldsEncoder::wrap(self.scratch)))
    }

    /// Copy the bytes of a value into the data buffer
    pub fn header_copy(mut self, t: &MessageHeader) -> CodecResult<SubscribedEventFieldsEncoder<'d>> {
        self.scratch.write_type::<MessageHeader>(t, 8)?;
        Ok(SubscribedEventFieldsEncoder::wrap(self.scratch))
    }
}

/// SubscribedEvent Encoder entry point
pub fn start_encoding_subscribed_event<'d>(data: &'d mut [u8]) -> SubscribedEventMessageHeaderEncoder<'d> {
    SubscribedEventMessageHeaderEncoder::wrap(ScratchEncoderData { data: data, pos: 0 })
}
