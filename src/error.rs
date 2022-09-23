use std::error;
use std::fmt;
use std::io;
use std::str;

/// A serialization error.
///
/// Can be converted to [`std::io::Error`].
pub struct Error {
    inner: io::Error,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<Error> for io::Error {
    fn from(error: Error) -> Self {
        error.inner
    }
}

impl error::Error for Error {
    #[allow(deprecated)]
    fn description(&self) -> &str {
        self.inner.description()
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        #[allow(deprecated)]
        self.inner.cause()
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.inner.source()
    }
}

impl serde::ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::other(msg.to_string())
    }
}

impl Error {
    pub(crate) fn new(inner: io::Error) -> Self {
        Self { inner }
    }

    pub(crate) fn invalid_data(inner: impl Into<Box<dyn error::Error + Send + Sync>>) -> Self {
        Self::new(io::Error::new(io::ErrorKind::InvalidData, inner))
    }

    pub(crate) fn invalid_input(inner: impl Into<Box<dyn error::Error + Send + Sync>>) -> Self {
        Self::new(io::Error::new(io::ErrorKind::InvalidInput, inner))
    }

    pub(crate) fn other(inner: impl Into<Box<dyn error::Error + Send + Sync>>) -> Self {
        Self::new(io::Error::new(io::ErrorKind::Other, inner))
    }
}

#[derive(Debug)]
pub(crate) enum Unexpected {
    Bool(bool),
    Unsigned(u128),
    Signed(i128),
    Float(f64),
    Char(char),
    Str,
    Bytes,
    Map(Option<usize>),
    Seq(Option<usize>),
    Struct(&'static str),
    Tuple(usize),
    Variant(&'static str, &'static str),
}

impl fmt::Display for Unexpected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unexpected::Bool(bool) => fmt::Debug::fmt(bool, f),
            Unexpected::Map(None) => write!(f, "map"),
            Unexpected::Map(Some(len)) => write!(f, "map of len {len}"),
            Unexpected::Seq(None) => write!(f, "seq"),
            Unexpected::Seq(Some(len)) => write!(f, "seq of len {len}"),
            Unexpected::Struct(name) => write!(f, "struct {name}"),
            Unexpected::Tuple(len) => write!(f, "tuple of len {len}"),
            Unexpected::Variant(ty, name) => write!(f, "variant {ty}::{name}"),
            Unexpected::Unsigned(_) => todo!(),
            Unexpected::Signed(_) => todo!(),
            Unexpected::Float(_) => todo!(),
            Unexpected::Char(_) => todo!(),
            Unexpected::Str => todo!(),
            Unexpected::Bytes => todo!(),
        }
    }
}