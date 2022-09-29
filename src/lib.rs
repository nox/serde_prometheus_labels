use serde::ser::{Serialize, Serializer};
use std::io::Write;

mod comma;
mod error;
mod top;
mod value;
mod str;

pub use self::error::Error;

/// Serializes `value` into a [`String`].
///
/// See [`serializer`] for information about the data format.
pub fn to_string(value: &impl Serialize) -> Result<String, Error> {
    let mut string = "".to_owned();

    value.serialize(top::TopSerializer::new(str::Writer::from_mut_string(
        &mut string,
    )))?;

    Ok(string)
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

/// A serializer for Prometheus labels.
///
/// This serializer only supports structs.
///
/// For struct fields, the supported values are scalars, strings, and bytes
/// that can be converted to strings. Nones and units are ignored, and unit
/// variants are serialized as their name. Anything else results in an error.
///
/// Prometheus labels are a sequence of comma-separated key-value pairs
/// as specified by the [Prometheus documentation][doc].
///
/// [doc]: https://github.com/prometheus/docs/blob/main/content/docs/instrumenting/exposition_formats.md#text-format-details
pub fn serializer(
    writer: &mut (impl ?Sized + Write),
) -> impl Serializer<Ok = (), Error = Error> + '_ {
    top::TopSerializer::new(str::Writer::new(writer))
}
