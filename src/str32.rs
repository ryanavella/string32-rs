use std::cmp;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::hash::{Hash, Hasher};

use usize_cast::IntoUsize;

use super::String32;
use super::TryFromStrError;

/// A slice of a `String32`.
///
/// This is just a thin wrapper around [`str`], but with the convenience of an API built around [`u32`] indices instead of [`usize`] indices.
#[derive(Debug, Eq)]
#[repr(transparent)]
pub struct Str32(str);

impl Str32 {
    /// Convert a `&Str32` to a [`&str`] slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::Str32;
    /// # use std::convert::TryInto;
    /// let s: &Str32 = "123".try_into().unwrap();
    /// assert_eq!("123", s.as_str());
    /// ```
    #[must_use]
    pub const fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert a `&mut Str32` to a `&mut str` slice.
    #[must_use]
    pub fn as_mut_str(&mut self) -> &mut str {
        &mut self.0
    }

    /// Converts the `Str32` to a byte slice.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Converts the `Str32` to a byte slice.
    ///
    /// # Safety
    ///
    /// See [`str::as_bytes_mut`].
    #[must_use]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.0.as_bytes_mut()
    }

    /// Returns an iterator over the bytes of the string slice.
    #[must_use]
    pub fn bytes(&self) -> std::str::Bytes<'_> {
        self.0.bytes()
    }

    /// Converts the `Str32` to a raw pointer.
    #[must_use]
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    /// Converts the `Str32` to a mutable raw pointer.
    ///
    /// The caller must ensure that the string slice is only modified in a way that
    /// ensures it is always valid UTF-8.
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }

    /// Converts a `&mut str` into a `&mut Str32`.
    ///
    /// # Panics
    ///
    /// Panics if the provided string slice occupies more than [`u32::MAX`] bytes.
    #[must_use]
    pub fn from_mut_str(s: &mut str) -> &mut Self {
        s.try_into().unwrap()
    }

    /// Returns the length of the `Str32` in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::Str32;
    /// # use std::convert::TryInto;
    /// let s: &Str32 = "test".try_into().unwrap();
    /// assert_eq!(4, s.len());
    /// ```
    #[must_use]
    pub fn len(&self) -> u32 {
        self.0.len().try_into().unwrap()
    }

    /// Returns whether the `Str32` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::Str32;
    /// # use std::convert::TryInto;
    /// let s: &Str32 = "".try_into().unwrap();
    /// assert!(s.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the characters of the `Str32`.
    #[must_use]
    pub fn chars(&self) -> std::str::Chars {
        self.0.chars()
    }

    /// Returns an iterator over the characters of the `Str32`, and their byte indices.
    #[must_use]
    pub fn char_indices(&self) -> impl DoubleEndedIterator<Item = (u32, char)> + '_ {
        self.0
            .char_indices()
            .map(|(i, c)| (i.try_into().unwrap(), c))
    }

    /// Returns an iterator over the lines of a `&Str32`.
    #[must_use]
    pub fn lines(&self) -> impl DoubleEndedIterator<Item = &Self> + '_ {
        self.0.lines().map(|line| line.try_into().unwrap())
    }

    /// Returns an iterator over the ASCII-whitespace-delimited words of a `&Str32`.
    #[must_use]
    pub fn split_ascii_whitespace(&self) -> impl DoubleEndedIterator<Item = &Self> + '_ {
        self.0
            .split_ascii_whitespace()
            .map(|line| line.try_into().unwrap())
    }

    /// Splits a `&Str32` in two at the given byte index.
    ///
    /// # Panics
    ///
    /// Panics when given an index that is not on a UTF-8 code point boundary, or if the index is out-of-bounds.
    #[must_use]
    pub fn split_at(&self, mid: u32) -> (&Self, &Self) {
        let (s1, s2) = self.0.split_at(mid.into_usize());
        (s1.try_into().unwrap(), s2.try_into().unwrap())
    }

    /// Splits a `&mut Str32` in two at the given byte index.
    ///
    /// # Panics
    ///
    /// Panics when given an index that is not on a UTF-8 code point boundary, or if the index is out-of-bounds.
    #[must_use]
    pub fn split_at_mut(&mut self, mid: u32) -> (&mut Self, &mut Self) {
        let (s1, s2) = self.0.split_at_mut(mid.into_usize());
        (s1.try_into().unwrap(), s2.try_into().unwrap())
    }

    /// Returns an iterator over the whitespace-delimited words of a `&Str32`.
    #[must_use]
    pub fn split_whitespace(&self) -> impl DoubleEndedIterator<Item = &Self> + '_ {
        self.0
            .split_whitespace()
            .map(|line| line.try_into().unwrap())
    }

    /// Checks if two string slices are equal, ignoring ASCII case mismatches.
    #[must_use]
    pub fn eq_ignore_ascii_case(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }

    /// Return an iterator over the string slice's chars, each escaped according to `char::escape_debug`.
    #[must_use]
    pub fn escape_debug(&self) -> std::str::EscapeDebug<'_> {
        self.0.escape_debug()
    }

    /// Return an iterator over the string slice's chars, each escaped according to `char::escape_default`.
    #[must_use]
    pub fn escape_default(&self) -> std::str::EscapeDefault<'_> {
        self.0.escape_default()
    }

    /// Return an iterator over the string slice's chars, each escaped according to `char::escape_unicode`.
    #[must_use]
    pub fn escape_unicode(&self) -> std::str::EscapeUnicode<'_> {
        self.0.escape_unicode()
    }

    /// Returns whether the given index corresponds to a `char` boundary.
    #[must_use]
    pub fn is_char_boundary(&self, index: u32) -> bool {
        self.0.is_char_boundary(index.into_usize())
    }

    /// Converts all uppercase ASCII characters to lowercase.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// # use std::convert::TryFrom;
    /// let mut s = String32::try_from("ABC").unwrap();
    /// s.make_ascii_lowercase();
    /// assert_eq!("abc", s);
    /// ```
    pub fn make_ascii_lowercase(&mut self) {
        self.0.make_ascii_lowercase()
    }

    /// Converts all lowercase ASCII characters to uppercase.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// # use std::convert::TryFrom;
    /// let mut s = String32::try_from("abc").unwrap();
    /// s.make_ascii_uppercase();
    /// assert_eq!("ABC", s);
    /// ```
    pub fn make_ascii_uppercase(&mut self) {
        self.0.make_ascii_uppercase()
    }

    /// Parses a `&Str32` slice into another type.
    ///
    /// # Errors
    ///
    /// Will return `Err` if this `&Str32` slice cannot be parsed into the desired type.
    ///
    /// `Err`: `string32::TryFromStringError`
    pub fn parse<F: std::str::FromStr>(&self) -> Result<F, F::Err> {
        self.0.parse()
    }

    /// Create a [`String32`] formed by `n` repetitions of this string slice.
    ///
    /// # Panics
    ///
    /// Panics if the resulting [`String32`] would require more than [`u32::MAX`] bytes.
    #[must_use]
    pub fn repeat(&self, n: u32) -> String32 {
        self.0.repeat(n.into_usize()).try_into().unwrap()
    }

    /// Returns a lowercase equivalent of this `&Str32` as a new [`String32`].
    #[must_use]
    pub fn to_lowercase(&self) -> String32 {
        self.0.to_lowercase().try_into().unwrap()
    }

    /// Returns an uppercase equivalent of this `&Str32` as a new [`String32`].
    #[must_use]
    pub fn to_uppercase(&self) -> String32 {
        self.0.to_uppercase().try_into().unwrap()
    }

    /// Returns a new [`String32`] with each ASCII uppercase character mapped to lowercase.
    #[must_use]
    pub fn to_ascii_lowercase(&self) -> String32 {
        self.0.to_ascii_lowercase().try_into().unwrap()
    }

    /// Returns a new [`String32`] with each ASCII lowercase character mapped to uppercase.
    #[must_use]
    pub fn to_ascii_uppercase(&self) -> String32 {
        self.0.to_ascii_uppercase().try_into().unwrap()
    }

    /// Returns a substring of this string with leading and trailing whitespace removed.
    #[must_use]
    pub fn trim(&self) -> &Self {
        self.0.trim().try_into().unwrap()
    }

    /// Returns a substring of this string with leading whitespace removed.
    #[must_use]
    pub fn trim_start(&self) -> &Self {
        self.0.trim_start().try_into().unwrap()
    }

    /// Returns a substring of this string with trailing whitespace removed.
    #[must_use]
    pub fn trim_end(&self) -> &Self {
        self.0.trim_end().try_into().unwrap()
    }

    /// Convert a `Box<Str32>` into a [`Box<str>`].
    ///
    /// This method has no overhead in the form of copying or allocating.
    #[must_use]
    pub fn into_boxed_str(self: Box<Self>) -> Box<str> {
        self.into()
    }

    /// Convert a `Box<Str32>` into `Box<[u8]>`.
    ///
    /// This method has no overhead in the form of copying or allocating.
    #[must_use]
    pub fn into_boxed_bytes(self: Box<Self>) -> Box<[u8]> {
        self.into()
    }

    /// Convert a `Box<Str32>` into a [`String`].
    ///
    /// This method has no overhead in the form of copying or allocating.
    #[must_use]
    pub fn into_string(self: Box<Self>) -> String {
        self.into()
    }

    /// Convert a `Box<Str32>` into a [`String32`].
    ///
    /// This method has no overhead in the form of copying or allocating.
    #[must_use]
    pub fn into_string32(self: Box<Self>) -> String32 {
        self.into()
    }
}

impl AsRef<[u8]> for Str32 {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsRef<str> for Str32 {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Str32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> From<&'a Str32> for &'a str {
    fn from(s: &'a Str32) -> Self {
        &s.0
    }
}

impl From<Box<Str32>> for String {
    fn from(b: Box<Str32>) -> Self {
        b.into()
    }
}

impl From<Box<Str32>> for Box<str> {
    fn from(b: Box<Str32>) -> Self {
        b.into()
    }
}

impl From<Box<Str32>> for Box<[u8]> {
    fn from(b: Box<Str32>) -> Self {
        b.into()
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<Box<Str32>> for String32 {
    fn from(b: Box<Str32>) -> Self {
        String::from(b).try_into().unwrap()
    }
}

impl Hash for Str32 {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0.hash(hasher);
    }
}

impl Ord for Str32 {
    fn cmp(&self, rhs: &Self) -> cmp::Ordering {
        self.0.cmp(&rhs.0)
    }
}

impl PartialEq for Str32 {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
}

impl PartialOrd for Str32 {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}

impl ToOwned for Str32 {
    type Owned = String32;

    fn to_owned(&self) -> String32 {
        self.0.to_owned().try_into().unwrap()
    }
}

impl<'a> TryFrom<&'a str> for &'a Str32 {
    type Error = TryFromStrError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        u32::try_from(s.len())
            .map(|_| {
                let ptr = s as *const str as *const Str32;
                unsafe {
                    // safety: relies on `&Str32` and `&str` having the same layout
                    &*ptr
                }
            })
            .map_err(|_| TryFromStrError(()))
    }
}

impl<'a> TryFrom<&'a mut str> for &'a mut Str32 {
    type Error = TryFromStrError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        u32::try_from(s.len())
            .map(|_| {
                let ptr = s as *mut str as *mut Str32;
                unsafe {
                    // safety: relies on `&mut Str32` and `&mut str` having the same layout
                    &mut *ptr
                }
            })
            .map_err(|_| TryFromStrError(()))
    }
}
