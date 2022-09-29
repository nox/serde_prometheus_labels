use std::{io, str};

/// A writer to which you can only write slices, through `Self::write_str`.
pub(crate) struct Writer<'io, W>
where
    W: ?Sized,
{
    inner: &'io mut W,
}

impl<'io> Writer<'io, Vec<u8>> {
    pub(crate) fn from_mut_string(s: &'io mut String) -> Self {
        // SAFETY: The only way to write to the IO sink is through methods
        // that take strings, which are guaranteed to be valid UTF-8.
        unsafe { Writer::new(s.as_mut_vec()) }
    }
}

impl<'io, W> Writer<'io, W>
where
    W: ?Sized + io::Write,
{
    pub(crate) fn new(inner: &'io mut W) -> Self {
        Self { inner }
    }

    pub(crate) fn reborrow(&mut self) -> Writer<'_, W> {
        Writer {
            inner: &mut *self.inner,
        }
    }

    pub(crate) fn write_str(&mut self, s: &str) -> io::Result<()> {
        self.inner.write_all(s.as_bytes())
    }
}

/// A pattern that is guaranteed to only contain ASCII chars.
#[derive(Clone, Copy)]
pub(crate) struct AsciiPattern {
    chars: &'static [u8],
}

impl AsciiPattern {
    /// Will fail to compile in a const context if the chars aren't ASCII.
    pub(crate) const fn new(chars: &'static [u8]) -> Self {
        #[allow(clippy::blocks_in_if_conditions)]
        if {
            let mut i = 0;
            loop {
                if i >= chars.len() {
                    break false;
                } else if chars[i] > 127 {
                    break true;
                }
                i += 1;
            }
        } {
            #[allow(unconditional_panic)]
            #[allow(clippy::no_effect)]
            ([] as [u8; 0])[0]; // Invalid ASCII chars
        }

        Self { chars }
    }

    pub(crate) fn take_until_match<'a>(self, haystack: &mut &'a str) -> Option<(&'a str, u8)> {
        let bytes = haystack.as_bytes();

        let chunk_end = bytes.iter().position(|c| self.chars.contains(c))?;

        // SAFETY: chunk_end is a char boundary, as bytes[chunk_end] is an ASCII char.
        let chunk = unsafe { str::from_utf8_unchecked(&bytes[..chunk_end]) };

        let found = bytes[chunk_end];

        // SAFETY: chunk_end is a char boundary, as bytes[chunk_end] is an ASCII char
        // and ASCII chars are always only 1 byte when encoded as UTF-8.
        *haystack = unsafe { str::from_utf8_unchecked(&bytes[chunk_end..][1..]) };

        Some((chunk, found))
    }
}
