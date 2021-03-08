use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::hash::Hash;
use std::slice;

use usize_cast::IntoUsize;

use super::String32;
use super::TryFromStringError;

/// A slice of a `String32`.
///
/// Should behave more or less the same as a `str` but some methods return `u32` instead of `usize`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Str32(str);

impl Str32 {
    /// Convert a `&Str32` to a `&str` slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
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

    /// Converts a `&str` into a `&Str32`.
    ///
    /// # Panics
    ///
    /// Panics if the provided string slice occupies more than `u32::MAX` bytes.
    #[must_use]
    pub fn from_str(s: &str) -> &Self {
        s.try_into().unwrap()
    }

    /// Converts a `&mut str` into a `&mut Str32`.
    ///
    /// # Panics
    ///
    /// Panics if the provided string slice occupies more than `u32::MAX` bytes.
    #[must_use]
    pub fn from_mut_str(s: &mut str) -> &mut Self {
        s.try_into().unwrap()
    }

    #[must_use]
    pub fn get<I: slice::SliceIndex<str>>(&self, i: I) -> Option<&I::Output> {
        self.0.get(i)
    }

    #[must_use]
    pub fn get_mut<I: slice::SliceIndex<str>>(&mut self, i: I) -> Option<&mut I::Output> {
        self.0.get_mut(i)
    }

    #[must_use]
    pub unsafe fn get_unchecked<I: slice::SliceIndex<str>>(&self, i: I) -> &I::Output {
        self.0.get_unchecked(i)
    }

    #[must_use]
    pub unsafe fn get_unchecked_mut<I: slice::SliceIndex<str>>(&mut self, i: I) -> &mut I::Output {
        self.0.get_unchecked_mut(i)
    }

    /// Returns the length of the `Str32` in bytes.
    #[must_use]
    pub fn len(&self) -> u32 {
        self.0.len().try_into().unwrap()
    }

    /// Returns whether the `Str32` is empty.
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

    pub fn lines(&self) -> impl DoubleEndedIterator<Item = &Str32> + '_ {
        self.0.lines().map(|line| line.try_into().unwrap())
    }

    pub fn split_ascii_whitespace(&self) -> impl DoubleEndedIterator<Item = &Str32> + '_ {
        self.0
            .split_ascii_whitespace()
            .map(|line| line.try_into().unwrap())
    }

    pub fn split_at(&self, mid: u32) -> (&Self, &Self) {
        let (s1, s2) = self.0.split_at(mid.into_usize());
        (s1.try_into().unwrap(), s2.try_into().unwrap())
    }

    pub fn split_at_mut(&mut self, mid: u32) -> (&mut Self, &mut Self) {
        let (s1, s2) = self.0.split_at_mut(mid.into_usize());
        (s1.try_into().unwrap(), s2.try_into().unwrap())
    }

    pub fn split_whitespace(&self) -> impl DoubleEndedIterator<Item = &Str32> + '_ {
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

    pub fn make_ascii_lowercase(&mut self) {
        self.0.make_ascii_lowercase()
    }

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

    #[must_use]
    pub fn repeat(&self, n: u32) -> String32 {
        self.0.repeat(n.into_usize()).try_into().unwrap()
    }

    #[must_use]
    pub fn to_lowercase(&self) -> String32 {
        self.0.to_lowercase().try_into().unwrap()
    }

    #[must_use]
    pub fn to_uppercase(&self) -> String32 {
        self.0.to_uppercase().try_into().unwrap()
    }

    #[must_use]
    pub fn to_ascii_lowercase(&self) -> String32 {
        self.0.to_ascii_lowercase().try_into().unwrap()
    }

    #[must_use]
    pub fn to_ascii_uppercase(&self) -> String32 {
        self.0.to_ascii_uppercase().try_into().unwrap()
    }

    #[must_use]
    pub fn trim(&self) -> &Self {
        self.0.trim().try_into().unwrap()
    }

    #[must_use]
    pub fn trim_start(&self) -> &Self {
        self.0.trim_start().try_into().unwrap()
    }

    #[must_use]
    pub fn trim_end(&self) -> &Self {
        self.0.trim_end().try_into().unwrap()
    }

    #[must_use]
    pub fn into_boxed_str(self: Box<Self>) -> Box<str> {
        self.into()
    }

    #[must_use]
    pub fn into_boxed_bytes(self: Box<Self>) -> Box<[u8]> {
        self.into()
    }

    #[must_use]
    pub fn into_string(self: Box<Self>) -> String {
        self.into()
    }

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

impl ToOwned for Str32 {
    type Owned = String32;

    fn to_owned(&self) -> String32 {
        self.0.to_owned().try_into().unwrap()
    }
}

impl From<Box<Str32>> for Box<str> {
    fn from(b: Box<Str32>) -> Self {
        let ptr = Box::into_raw(b) as *mut str;
        unsafe {
            // safety: relies on `Box<Str32>` and `Box<str>` having the same layout
            Box::from_raw(ptr)
        }
    }
}

impl From<Box<Str32>> for Box<[u8]> {
    fn from(b: Box<Str32>) -> Self {
        b.into_boxed_str().into_boxed_bytes()
    }
}

impl From<Box<Str32>> for String {
    fn from(b: Box<Str32>) -> Self {
        b.into_boxed_str().into_string()
    }
}

impl From<Box<Str32>> for String32 {
    fn from(b: Box<Str32>) -> Self {
        String::from(b).try_into().unwrap()
    }
}

impl<'a> TryFrom<&'a str> for &'a Str32 {
    type Error = TryFromStringError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        u32::try_from(s.len())
            .map(|_| {
                let ptr = s as *const str as *const Str32;
                unsafe {
                    // safety: relies on `&Str32` and `&str` having the same layout
                    &*ptr
                }
            })
            .map_err(|_| TryFromStringError(()))
    }
}

impl<'a> TryFrom<&'a mut str> for &'a mut Str32 {
    type Error = TryFromStringError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        u32::try_from(s.len())
            .map(|_| {
                let ptr = s as *mut str as *mut Str32;
                unsafe {
                    // safety: relies on `&mut Str32` and `&mut str` having the same layout
                    &mut *ptr
                }
            })
            .map_err(|_| TryFromStringError(()))
    }
}
