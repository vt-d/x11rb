use std::convert::{TryFrom, TryInto};

use crate::utils::Buffer;
use crate::errors::ParseError;

/// Common information on events and errors.
///
/// This trait exists to share some code between `GenericEvent` and `GenericError`.
pub trait Event {
    /// Provide the raw data of the event as a slice.
    fn raw_bytes(&self) -> &[u8];

    /// The raw type of this response.
    ///
    /// Response types have seven bits in X11. The eight bit indicates whether this packet was
    /// generated through the `SendEvent` request.
    ///
    /// See also the `response_type()` and `server_generated()` methods which decompose this field
    /// into the contained information.
    fn raw_response_type(&self) -> u8 {
        self.raw_bytes()[0]
    }

    /// The type of this response.
    ///
    /// All errors have a response type of 0. Replies have a response type of 1, but you should
    /// never see their raw bytes in your code. Other response types are provided as constants in
    /// the generated code. Note that extensions have their response type dynamically assigned.
    fn response_type(&self) -> u8 {
        self.raw_response_type() & 0x7f
    }

    /// Was this packet generated by the server?
    ///
    /// If this function returns true, then this event comes from the X11 server. Otherwise, it was
    /// sent from another client via the `SendEvent` request.
    fn server_generated(&self) -> bool {
        self.raw_response_type() & 0x80 == 0
    }

    /// Get the sequence number of this packet.
    ///
    /// Not all packets contain a sequence number, so this function returns an `Option`.
    fn raw_sequence_number(&self) -> Option<u16> {
        use crate::generated::xproto::KEYMAP_NOTIFY_EVENT;
        match self.response_type() {
            KEYMAP_NOTIFY_EVENT => None,
            _ => {
                let bytes = self.raw_bytes();
                Some(u16::from_ne_bytes([bytes[2], bytes[3]]))
            }
        }
    }
}

/// A generic event.
///
/// Examine the event's `response_type()` and use `TryInto::try_into()` to convert the event to the
/// desired type.
#[derive(Debug, Clone)]
pub struct GenericEvent(Buffer);

impl Event for GenericEvent {
    fn raw_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Into<Buffer> for GenericEvent {
    fn into(self) -> Buffer {
        self.0
    }
}

const REPLY: u8 = 1;

impl TryFrom<Buffer> for GenericEvent {
    type Error = ParseError;

    fn try_from(value: Buffer) -> Result<Self, Self::Error> {
        use super::generated::xproto::GE_GENERIC_EVENT;
        if value.len() < 32 {
            return Err(ParseError::ParseError);
        }
        let length_field = u32::from_ne_bytes([value[4], value[5], value[6], value[7]]);
        let length_field: usize = length_field.try_into()?;
        let actual_length = value.len();
        let event = GenericEvent(value);
        let expected_length = match event.response_type() {
            GE_GENERIC_EVENT | REPLY => 32 + 4 * length_field,
            _ => 32
        };
        if actual_length != expected_length {
            return Err(ParseError::ParseError);
        }
        Ok(event)
    }
}

impl From<GenericError> for GenericEvent {
    fn from(value: GenericError) -> Self {
        GenericEvent(value.into())
    }
}

/// A generic error.
///
/// This struct is similar to `GenericEvent`, but is specific to error packets. It allows access to
/// the contained error code. This error code allows you to pick the right error type for
/// conversion via `TryInto::try_into()`.
#[derive(Debug, Clone)]
pub struct GenericError(Buffer);

impl GenericError {
    /// Get the error code of this error.
    ///
    /// The error code identifies what kind of error this packet contains. Note that extensions
    /// have their error codes dynamically assigned.
    pub fn error_code(&self) -> u8 {
        self.raw_bytes()[1]
    }
}

impl Event for GenericError {
    fn raw_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Into<Buffer> for GenericError {
    fn into(self) -> Buffer {
        self.0
    }
}

impl TryFrom<GenericEvent> for GenericError {
    type Error = ParseError;

    fn try_from(event: GenericEvent) -> Result<Self, Self::Error> {
        if event.response_type() != 0 {
            return Err(ParseError::ParseError)
        }
        Ok(GenericError(event.into()))
    }
}

impl TryFrom<Buffer> for GenericError {
    type Error = ParseError;

    fn try_from(value: Buffer) -> Result<Self, Self::Error> {
        let event: GenericEvent = value.try_into()?;
        event.try_into()
    }
}

/// A type implementing this trait can be parsed from some raw bytes.
pub trait TryParse: Sized {
    /// Try to parse the given values into an instance of this type.
    ///
    /// If parsing is successful, an instance of the type and a slice for the remaining data should
    /// be returned. Otherwise, an error is returned.
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError>;
}

/// A type implementing this trait can be serialized into X11 raw bytes.
pub trait Serialize {
    /// The value returned by `serialize`.
    ///
    /// This should be `Vec<u8>` in most cases. However, arrays like `[u8; 4]` should also be
    /// allowed and thus this is an associated type.
    ///
    /// If generic associated types were available, implementing `AsRef<[u8]>` would be required.
    type Bytes;

    /// Serialize this value into X11 raw bytes.
    fn serialize(&self) -> Self::Bytes;
}

// Now implement TryParse and Serialize for some primitive data types that we need.

macro_rules! implement_try_parse {
    ($t:ty: [$($indicies: expr),*]) => {
        impl TryParse for $t {
            fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
                let len = std::mem::size_of::<$t>();
                if value.len() < len {
                    Err(ParseError::ParseError)
                } else {
                    Ok((<$t>::from_ne_bytes([ $(value[$indicies],)* ]), &value[len..]))
                }
            }
        }
    }
}

macro_rules! implement_serialize {
    ($t:ty: $size:expr) => {
        impl Serialize for $t {
            type Bytes = [u8; $size];
            fn serialize(&self) -> Self::Bytes {
                self.to_ne_bytes()
            }
        }
    }
}

macro_rules! forward_float {
    ($from:ty: $to:ty) => {
        impl TryParse for $from {
            fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
                let (data, remaining) = <$to>::try_parse(value)?;
                Ok((<$from>::from_bits(data), remaining))
            }
        }
        impl Serialize for $from {
            type Bytes = <$to as Serialize>::Bytes;
            fn serialize(&self) -> Self::Bytes {
                self.to_bits().serialize()
            }
        }
    }
}

implement_try_parse!(u8: [0]);
implement_try_parse!(i8: [0]);
implement_try_parse!(u16: [0, 1]);
implement_try_parse!(i16: [0, 1]);
implement_try_parse!(u32: [0, 1, 2, 3]);
implement_try_parse!(i32: [0, 1, 2, 3]);
implement_try_parse!(u64: [0, 1, 2, 3, 4, 5, 6, 7]);
implement_try_parse!(i64: [0, 1, 2, 3, 4, 5, 6, 7]);

implement_serialize!(u8: 1);
implement_serialize!(i8: 1);
implement_serialize!(u16: 2);
implement_serialize!(i16: 2);
implement_serialize!(u32: 4);
implement_serialize!(i32: 4);
implement_serialize!(u64: 8);
implement_serialize!(i64: 8);

forward_float!(f32: u32);
forward_float!(f64: u64);

impl TryParse for bool {
    fn try_parse(value: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (data, remaining) = u8::try_parse(value)?;
        Ok((data != 0, remaining))
    }
}

impl Serialize for bool {
    type Bytes = [u8; 1];
    fn serialize(&self) -> Self::Bytes {
        [*self as u8]
    }
}

impl<T> Serialize for [T]
where T: Serialize,
      <T as Serialize>::Bytes: AsRef<[u8]>
{
    type Bytes = Vec<u8>;
    fn serialize(&self) -> Self::Bytes {
        let mut result = Vec::new();
        for item in self {
            result.extend(item.serialize().as_ref());
        }
        result
    }
}

// This macro is used by the generated code to implement `std::ops::BitOr` and
// `std::ops::BitOrAssign`.
macro_rules! bitmask_binop {
    ($t:ty, $u:ty) => {
        impl std::ops::BitOr for $t {
            type Output = $u;
            fn bitor(self, other: Self) -> Self::Output {
                Into::<Self::Output>::into(self) | Into::<Self::Output>::into(other)
            }
        }
        impl std::ops::BitOr<$u> for $t {
            type Output = $u;
            fn bitor(self, other: $u) -> Self::Output {
                Into::<Self::Output>::into(self) | other
            }
        }
        impl std::ops::BitOr<$t> for $u {
            type Output = $u;
            fn bitor(self, other: $t) -> Self::Output {
                self | Into::<Self::Output>::into(other)
            }
        }
        impl std::ops::BitOrAssign<$t> for $u {
            fn bitor_assign(&mut self, other: $t) {
                *self |= Into::<Self>::into(other)
            }
        }
    }
}
