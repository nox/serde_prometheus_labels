use serde::ser::Serialize;
use std::io::Write;

mod comma;
mod error;
mod top;
mod value;

pub use self::error::Error;
pub use self::top::serializer;

/// Serializes `value` into a [`String`].
///
/// See [`serializer`] for information about the data format.
pub fn to_string(value: &impl Serialize) -> Result<String, Error> {
    let vec = to_vec(value)?;

    Ok(if cfg!(debug_assertions) {
        String::from_utf8(vec).expect("valid UTF-8")
    } else {
        unsafe { String::from_utf8_unchecked(vec) }
    })
}

/// Serializes `value` into a [`Vec<u8>`].
///
/// See [`serializer`] for information about the data format.
pub fn to_vec(value: &impl Serialize) -> Result<Vec<u8>, Error> {
    let mut buf = vec![];
    to_writer(&mut buf, value)?;
    Ok(buf)
}

/// Serializes `value` into [`writer`][Write].
///
/// See [`serializer`] for information about the data format.
pub fn to_writer(writer: &mut (impl ?Sized + Write), value: &impl Serialize) -> Result<(), Error> {
    value.serialize(serializer(writer))
}
