use crate::comma::{ShouldWriteComma, WroteAnything};
use crate::error::{Error, Unexpected};
use crate::str::{AsciiPattern, Writer};
use serde::ser::{Impossible, Serialize, Serializer};
use std::{error, fmt, io, str};

#[inline]
pub(crate) fn serializer<'w, W>(
    writer: Writer<'w, W>,
    key: &'static str,
    should_write_comma: ShouldWriteComma,
) -> impl 'w + Serializer<Ok = WroteAnything, Error = Error>
where
    W: ?Sized + io::Write,
{
    ValueSerializer {
        should_write_comma,
        key,
        writer,
    }
}

struct ValueSerializer<'w, W>
where
    W: ?Sized,
{
    writer: Writer<'w, W>,
    key: &'static str,
    should_write_comma: ShouldWriteComma,
}

macro_rules! delegate {
    ($($delegate:ident { $($($method:ident: $ty:ty),+ $(,)?)? })*) => {$($($(
        #[inline]
        fn $method(self, v: $ty) -> Result<Self::Ok, Error> {
            self.$delegate(v)
        }
    )?)*)*}
}

impl<'w, W> Serializer for ValueSerializer<'w, W>
where
    W: ?Sized + io::Write,
{
    type Ok = WroteAnything;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Error>;
    type SerializeTuple = Impossible<Self::Ok, Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Error>;
    type SerializeMap = Impossible<Self::Ok, Error>;
    type SerializeStruct = Impossible<Self::Ok, Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Error>;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Error> {
        self.write_key()?;
        self.write_unchecked(if v { r#"="true""# } else { r#"="false""# })?;

        Ok(WroteAnything(true))
    }

    delegate! {
        serialize_integer {
            serialize_i8: i8,
            serialize_i16: i16,
            serialize_i32: i32,
            serialize_i64: i64,
            serialize_u8: u8,
            serialize_u16: u16,
            serialize_u32: u32,
            serialize_u64: u64,
            serialize_u128: u128,
            serialize_i128: i128,
        }

        serialize_floating {
            serialize_f32: f32,
            serialize_f64: f64,
        }

        serialize_str {
            serialize_unit_struct: &'static str,
        }
    }

    fn serialize_char(mut self, v: char) -> Result<Self::Ok, Error> {
        self.write_key()?;

        self.write_unchecked(match v {
            '"' => r#"="\"""#,
            '\\' => r#"="\\""#,
            '\n' => r#"="\n""#,
            _ => {
                let mut buf = [0; 4];
                let part = v.encode_utf8(&mut buf);

                self.write_unchecked(r#"=""#)?;
                self.write_unchecked(part)?;

                return self.end_value();
            }
        })?;

        Ok(WroteAnything(true))
    }

    fn serialize_str(mut self, value: &str) -> Result<Self::Ok, Error> {
        self.begin_value()?;
        write_escaped(self.writer.reborrow(), value).map_err(Error::new)?;
        self.end_value()
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Error> {
        Err(self.unexpected(Unexpected::Bytes))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Error> {
        Ok(WroteAnything(false))
    }

    fn serialize_unit_variant(
        self,
        _ty: &'static str,
        _index: u32,
        name: &'static str,
    ) -> Result<Self::Ok, Error> {
        self.serialize_str(name)
    }

    fn serialize_newtype_struct<T>(self, _ty: &'static str, value: &T) -> Result<Self::Ok, Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        ty: &'static str,
        _index: u32,
        name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Error>
    where
        T: ?Sized + Serialize,
    {
        Err(self.unexpected(Unexpected::Variant(ty, name)))
    }

    fn serialize_none(self) -> Result<Self::Ok, Error> {
        Ok(WroteAnything(false))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Err(self.unexpected(Unexpected::Seq(len)))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Error> {
        Err(self.unexpected(Unexpected::Tuple(len)))
    }

    fn serialize_tuple_struct(
        self,
        ty: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTuple, Error> {
        Err(self.unexpected(Unexpected::Struct(ty)))
    }

    fn serialize_tuple_variant(
        self,
        ty: &'static str,
        _index: u32,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Err(self.unexpected(Unexpected::Variant(ty, name)))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Err(self.unexpected(Unexpected::Map(len)))
    }

    fn serialize_struct(
        self,
        ty: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Err(self.unexpected(Unexpected::Struct(ty)))
    }

    fn serialize_struct_variant(
        self,
        ty: &'static str,
        _index: u32,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Err(self.unexpected(Unexpected::Variant(ty, name)))
    }

    fn collect_str<T>(mut self, value: &T) -> Result<Self::Ok, Error>
    where
        T: ?Sized + fmt::Display,
    {
        struct Adapter<'w, W>
        where
            W: ?Sized,
        {
            writer: Writer<'w, W>,
            error: Option<Error>,
        }

        impl<'w, W> fmt::Write for Adapter<'w, W>
        where
            W: ?Sized + io::Write,
        {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                debug_assert!(self.error.is_none());

                write_escaped(self.writer.reborrow(), s).map_err(|err| {
                    self.error = Some(Error::new(err));

                    fmt::Error
                })
            }
        }

        self.begin_value()?;

        {
            let mut adapter = Adapter {
                writer: self.writer.reborrow(),
                error: None,
            };

            match fmt::write(&mut adapter, format_args!("{}", value)) {
                Ok(()) => debug_assert!(adapter.error.is_none()),
                Err(fmt::Error) => {
                    return Err(adapter.error.expect("there should be an error"));
                }
            }
        }

        self.end_value()
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

impl<'w, W> ValueSerializer<'w, W>
where
    W: ?Sized + io::Write,
{
    fn serialize_integer<I>(mut self, value: I) -> Result<WroteAnything, Error>
    where
        I: itoa::Integer,
    {
        let mut buf = itoa::Buffer::new();
        let part = buf.format(value);

        self.begin_value()?;
        self.write_unchecked(part)?;
        self.end_value()
    }

    fn serialize_floating<F>(mut self, value: F) -> Result<WroteAnything, Error>
    where
        F: ryu::Float,
    {
        let mut buf = ryu::Buffer::new();
        let part = buf.format(value);

        self.begin_value()?;
        self.write_unchecked(part)?;
        self.end_value()
    }

    fn begin_value(&mut self) -> Result<(), Error> {
        self.write_key()?;
        self.write_unchecked(r#"=""#)
    }

    fn write_key(&mut self) -> Result<(), Error> {
        if self.should_write_comma.0 {
            self.write_unchecked(",")?;
        }

        self.writer.write_str(self.key).map_err(Error::new)
    }

    fn write_unchecked(&mut self, raw: &str) -> Result<(), Error> {
        self.writer.write_str(raw).map_err(Error::new)
    }

    fn end_value(&mut self) -> Result<WroteAnything, Error> {
        self.write_unchecked(r#"""#)?;

        Ok(WroteAnything(true))
    }

    fn unexpected(&self, kind: Unexpected) -> Error {
        #[derive(Debug)]
        struct UnexpectedValueError(Unexpected);

        impl error::Error for UnexpectedValueError {
            #[allow(deprecated)]
            fn description(&self) -> &str {
                "unexpected value"
            }
        }

        impl fmt::Display for UnexpectedValueError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "unexpected {}", self.0)
            }
        }

        Error::invalid_input(UnexpectedValueError(kind))
    }
}

fn write_escaped(
    mut writer: Writer<'_, impl ?Sized + io::Write>,
    mut s: &str,
) -> Result<(), io::Error> {
    const PATTERN: AsciiPattern = AsciiPattern::new(b"\"\\\n");

    while let Some((chunk, found)) = PATTERN.take_until_match(&mut s) {
        writer.write_str(chunk)?;

        let escape_buf: [u8; 2];

        writer.write_str(if found == b'\n' {
            r#"\n"#
        } else {
            escape_buf = [b'\\', found];

            // SAFETY: We know that `found` is an ASCII char, so `escape_buf`
            // contains valid UTF-8.
            unsafe { std::str::from_utf8_unchecked(&escape_buf) }
        })?;
    }

    writer.write_str(s)
}
