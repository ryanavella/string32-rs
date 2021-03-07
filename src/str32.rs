use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::hash::Hash;

use usize_cast::IntoUsize;

use super::util;
use super::CharIndices;
use super::String32;
use super::TryFromStringError;

/// A slice of a `String32`.
///
/// Should behave more or less the same as a `str` but some methods return `u32` instead of `usize`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Str32 {
    slice: [u8],
}

impl Str32 {
    /// Convert a `&Str32` to a `&str` slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        unsafe {
            // safety: Str32 only holds valid UTF-8
            util::str_from_utf8_unchecked(&self.slice)
        }
    }

    /// Convert a `&mut Str32` to a `&mut str` slice.
    #[must_use]
    pub fn as_mut_str(&mut self) -> &mut str {
        unsafe {
            // safety: Str32 only holds valid UTF-8
            util::str_from_utf8_unchecked_mut(&mut self.slice)
        }
    }

    /// Converts the `Str32` to a byte slice.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.slice
    }

    /// Converts the `Str32` to a raw pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *const u8 {
        self.slice.as_ptr()
    }

    /// Converts the `Str32` to a mutable raw pointer.
    ///
    /// The caller must ensure that the string slice is only modified in a way that
    /// ensures it is always valid UTF-8.
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.slice.as_mut_ptr()
    }

    pub fn from_mut_str(s: &mut str) -> &mut Self {
        let _ = u32::try_from(s.len()).unwrap();
        unsafe {
            // safety: relies on `&mut Str32` and `&mut [u8]` having the same layout. (todo: is there a better way?)
            &mut *(s.as_bytes_mut() as *mut [u8] as *mut Self)
        }
    }

    /// Returns the length of the `Str32` in bytes.
    #[must_use]
    pub fn len(&self) -> u32 {
        self.slice.len().try_into().unwrap()
    }

    /// Returns whether the `Str32` is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the characters of the `Str32`.
    #[must_use]
    pub fn chars(&self) -> std::str::Chars {
        self.as_str().chars()
    }

    /// Returns an iterator over the characters of the `Str32`, and their byte indices.
    #[must_use]
    pub fn char_indices(&self) -> CharIndices {
        CharIndices::from(self)
    }

    #[must_use]
    pub fn is_char_boundary(&self, index: u32) -> bool {
        self.as_str().is_char_boundary(index.into_usize())
    }

    pub fn make_ascii_lowercase(&mut self) {
        self.as_mut_str().make_ascii_lowercase()
    }

    pub fn make_ascii_uppercase(&mut self) {
        self.as_mut_str().make_ascii_uppercase()
    }

    /// Parses a `&Str32` slice into another type.
    ///
    /// # Errors
    ///
    /// Will return `Err` if this `&Str32` slice cannot be parsed into the desired type.
    ///
    /// `Err`: `string32::TryFromStringError`
    pub fn parse<F: std::str::FromStr>(&self) -> Result<F, F::Err> {
        std::str::FromStr::from_str(self.as_str())
    }

    #[must_use]
    pub fn repeat(&self, n: u32) -> String32 {
        self.as_str().repeat(n.into_usize()).try_into().unwrap()
    }

    #[must_use]
    pub fn to_lowercase(&self) -> String32 {
        self.as_str().to_lowercase().try_into().unwrap()
    }

    #[must_use]
    pub fn to_uppercase(&self) -> String32 {
        self.as_str().to_uppercase().try_into().unwrap()
    }

    #[must_use]
    pub fn to_ascii_lowercase(&self) -> String32 {
        self.as_str().to_ascii_lowercase().try_into().unwrap()
    }

    #[must_use]
    pub fn to_ascii_uppercase(&self) -> String32 {
        self.as_str().to_ascii_uppercase().try_into().unwrap()
    }

    #[must_use]
    pub fn trim(&self) -> &Self {
        self.as_str().trim().try_into().unwrap()
    }

    #[must_use]
    pub fn trim_start(&self) -> &Self {
        self.as_str().trim_start().try_into().unwrap()
    }

    #[must_use]
    pub fn trim_end(&self) -> &Self {
        self.as_str().trim_end().try_into().unwrap()
    }
}

impl AsRef<[u8]> for Str32 {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for Str32 {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Str32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl ToOwned for Str32 {
    type Owned = String32;

    fn to_owned(&self) -> String32 {
        let s = unsafe {
            // safety: Str32 only holds valid UTF-8
            util::str_from_utf8_unchecked(&self.slice)
        };
        s.to_owned().try_into().unwrap()
    }
}

impl TryFrom<&str> for &Str32 {
    type Error = TryFromStringError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match u32::try_from(s.len()) {
            Ok(_) => Ok(unsafe {
                // safety: relies on `&Str32` and `&[u8]` having the same layout. (todo: is there a better way?)
                &*(s.as_bytes() as *const [u8] as *const Str32)
            }),
            Err(_) => Err(TryFromStringError(())),
        }
    }
}
