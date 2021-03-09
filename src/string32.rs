use std::borrow::{Borrow, BorrowMut};
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter;
use std::mem;
use std::ops;

use mediumvec::Vec32;
use usize_cast::IntoUsize;

use super::Str32;
use super::TryFromStringError;

/// A string that is indexed by `u32` instead of `usize`.
///
/// On 64-bit platforms, `String32` only requires 16 bytes to store the pointer, length, and capacity. [`String`] by comparison requires 24 bytes, plus padding.
#[derive(Clone, Debug, Default, PartialOrd, Eq, Ord)]
#[repr(transparent)]
pub struct String32(Vec32<u8>);

impl String32 {
    /// Creates a new empty `String32`.
    #[must_use]
    pub fn new() -> Self {
        Self(Vec32::new())
    }

    /// Creates a new empty `String32` with given capacity.
    #[must_use]
    pub fn with_capacity(cap: u32) -> Self {
        Self(Vec32::with_capacity(cap))
    }

    /// Returns the length of this `String32` in bytes.
    #[must_use]
    pub fn len(&self) -> u32 {
        self.0.len().try_into().unwrap()
    }

    /// Returns the capacity of this `String32` in bytes.
    #[must_use]
    pub fn capacity(&self) -> u32 {
        self.0.capacity()
    }

    /// Return whether the `String32` is an empty string.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// A helper to call arbitrary [`String`] methods on a `String32.`
    ///
    /// # Panics
    ///
    /// Panics if the resulting string would require more than [`u32::MAX`] bytes.
    pub fn as_string<F, T>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut String) -> T,
    {
        let mut s = mem::take(self).into();
        let ret = f(&mut s);
        *self = s.try_into().unwrap();
        ret
    }

    /// Push a `char` to the end of this `String32`.
    ///
    /// # Panics
    ///
    /// Panics if the resulting string would require more than [`u32::MAX`] bytes.
    pub fn push(&mut self, ch: char) {
        self.as_string(|s| s.push(ch));
    }

    /// Append a string slice to the end of this `String32`.
    ///
    /// # Panics
    ///
    /// Panics if the resulting string would require more than [`u32::MAX`] bytes.
    pub fn push_str<S>(&mut self, string: S)
    where
        S: AsRef<str>,
    {
        self.as_string(|s| s.push_str(string.as_ref()));
    }

    /// Pop a `char` from the end of this `String32`.
    pub fn pop(&mut self) -> Option<char> {
        self.as_string(String::pop)
    }

    /// Return the `char` at a given byte index.
    pub fn remove(&mut self, idx: u32) -> char {
        self.as_string(|s| s.remove(idx.into_usize()))
    }

    /// Insert a `char` at a given byte index.
    ///
    /// # Panics
    ///
    /// Panics if the resulting string would require more than [`u32::MAX`] bytes.
    pub fn insert(&mut self, idx: u32, ch: char) {
        self.as_string(|s| s.insert(idx.into_usize(), ch));
    }

    /// Insert a string slice at the given byte index.
    ///
    /// # Panics
    ///
    /// Panics if the resulting string would require more than [`u32::MAX`] bytes.
    pub fn insert_str<S>(&mut self, idx: u32, string: S)
    where
        S: AsRef<str>,
    {
        self.as_string(|s| s.insert_str(idx.into_usize(), string.as_ref()));
    }

    /// Reserve space for additional bytes.
    pub fn reserve(&mut self, additional: u32) {
        self.0.reserve(additional)
    }

    /// Reserve space for an exact number of bytes.
    pub fn reserve_exact(&mut self, additional: u32) {
        self.0.reserve_exact(additional)
    }

    /// Shrink the capacity of this `String32` to match its length.
    pub fn shrink_to_fit(&mut self) {
        self.as_string(String::shrink_to_fit);
    }

    /// Shortens this `String32` to the specified length.
    pub fn truncate(&mut self, new_len: u32) {
        self.as_string(|s| s.truncate(new_len.into_usize()));
    }

    /// Truncates the `String32` into an empty string.
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Converts a `String32` into a vector of bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0.into_vec()
    }

    /// Returns a string slice encompassing the entire `String32`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        if cfg!(debug_assertions) {
            std::str::from_utf8(&self.0).unwrap()
        } else {
            unsafe {
                // safety: we never store a non-utf8 Vec32<u8> in a String32
                std::str::from_utf8_unchecked(&self.0)
            }
        }
    }

    /// Returns a *mutable* string slice encompassing the entire `String32`.
    #[must_use]
    pub fn as_mut_str(&mut self) -> &mut str {
        if cfg!(debug_assertions) {
            std::str::from_utf8_mut(&mut self.0).unwrap()
        } else {
            unsafe {
                // safety: we never store a non-utf8 Vec32<u8> in a String32
                std::str::from_utf8_unchecked_mut(&mut self.0)
            }
        }
    }

    /// Return a byte slice of the `String32`'s contents.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Converts a `String32` into a [`Box<str>`].
    #[must_use]
    pub fn into_boxed_str(self) -> Box<str> {
        String::from(self).into_boxed_str()
    }

    /// Splits the `String32` into two at the given byte index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out-of-bounds or is not a UTF-8 code point boundary.
    pub fn split_off(&mut self, at: u32) -> Self {
        self.as_string(|s| s.split_off(at.into_usize()).try_into().unwrap())
    }

    /// Create a new `String32` from a raw pointer and corresponding length/capacity.
    ///
    /// # Safety
    ///
    /// See [`String::from_raw_parts`].
    pub unsafe fn from_raw_parts(buf: *mut u8, len: u32, cap: u32) -> Self {
        Self(Vec32::from_vec(Vec::from_raw_parts(
            buf,
            len.into_usize(),
            cap.into_usize(),
        )))
    }

    /// Decodes a UTF-8 encoded vector of bytes into a `String32`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the slice is not valid UTF-8.
    ///
    /// # Panics
    ///
    /// Panics if the provided [`Vec<u8>`] holds more than [`u32::MAX`] bytes.
    pub fn from_utf8(v: Vec<u8>) -> Result<Self, std::string::FromUtf8Error> {
        String::from_utf8(v).map(|s| s.try_into().unwrap())
    }

    /// Decodes a UTF-16 encoded slice into a `String32`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the slice is not valid UTF-16.
    ///
    /// # Panics
    ///
    /// Panics if the resulting UTF-8 representation would require more than [`u32::MAX`] bytes.
    pub fn from_utf16(v: &[u16]) -> Result<Self, std::string::FromUtf16Error> {
        String::from_utf16(v).map(|s| s.try_into().unwrap())
    }

    /// Lossily decodes a UTF-16 encoded slice into a `String32`.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the slice is not valid UTF-16.
    ///
    /// # Panics
    ///
    /// Panics if the resulting UTF-8 representation would require more than [`u32::MAX`] bytes.
    #[must_use]
    pub fn from_utf16_lossy(v: &[u16]) -> Self {
        String::from_utf16_lossy(v).try_into().unwrap()
    }

    /// Converts all uppercase ASCII characters to lowercase.
    pub fn make_ascii_lowercase(&mut self) {
        self.as_string(|s| s.make_ascii_lowercase());
    }

    /// Converts all lowercase ASCII characters to uppercase.
    pub fn make_ascii_uppercase(&mut self) {
        self.as_string(|s| s.make_ascii_uppercase());
    }
}

impl ops::Add<&str> for String32 {
    type Output = Self;

    #[must_use]
    fn add(mut self, rhs: &str) -> Self {
        self.push_str(rhs);
        self
    }
}

impl ops::Add<&Str32> for String32 {
    type Output = Self;

    #[must_use]
    fn add(mut self, rhs: &Str32) -> Self {
        self.push_str(rhs);
        self
    }
}

impl ops::AddAssign<&str> for String32 {
    fn add_assign(&mut self, rhs: &str) {
        self.push_str(rhs);
    }
}

impl ops::AddAssign<&Str32> for String32 {
    fn add_assign(&mut self, rhs: &Str32) {
        self.push_str(rhs);
    }
}

impl AsMut<Str32> for String32 {
    fn as_mut(&mut self) -> &mut Str32 {
        self
    }
}

impl AsRef<Str32> for String32 {
    fn as_ref(&self) -> &Str32 {
        &*self
    }
}

impl AsRef<[u8]> for String32 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<str> for String32 {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<Str32> for String32 {
    fn borrow(&self) -> &Str32 {
        &*self
    }
}

impl BorrowMut<Str32> for String32 {
    fn borrow_mut(&mut self) -> &mut Str32 {
        &mut *self
    }
}

impl ops::Deref for String32 {
    type Target = Str32;

    fn deref(&self) -> &Str32 {
        unsafe {
            // safety: relies on [`&Str32`] and `&[u8]` having the same layout. (todo: is there a better way?)
            &*(self.as_str().as_bytes() as *const [u8] as *const Str32)
        }
    }
}

impl ops::DerefMut for String32 {
    fn deref_mut(&mut self) -> &mut Str32 {
        unsafe {
            // safety: relies on `&mut Str32` and `&mut [u8]` having the same layout. (todo: is there a better way?)
            &mut *(self.as_mut_str().as_bytes_mut() as *mut [u8] as *mut Str32)
        }
    }
}

impl fmt::Display for String32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl From<&Str32> for String32 {
    #[inline]
    fn from(s: &Str32) -> Self {
        s.to_owned()
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<String32> for String {
    fn from(s: String32) -> Self {
        if cfg!(debug_assertions) {
            Self::from_utf8(s.0.into_vec()).unwrap()
        } else {
            unsafe {
                // safety: we never store a non-utf8 Vec32<u8> in a String32
                Self::from_utf8_unchecked(s.0.into_vec())
            }
        }
    }
}

impl From<String32> for Vec<u8> {
    fn from(s: String32) -> Self {
        s.0.into_vec()
    }
}

impl iter::FromIterator<char> for String32 {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        String::from_iter(iter).try_into().unwrap()
    }
}

impl<'a> iter::FromIterator<&'a char> for String32 {
    fn from_iter<I: IntoIterator<Item = &'a char>>(iter: I) -> Self {
        String::from_iter(iter).try_into().unwrap()
    }
}

impl<'a> iter::FromIterator<&'a str> for String32 {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        String::from_iter(iter).try_into().unwrap()
    }
}

impl<'a> iter::FromIterator<&'a Str32> for String32 {
    fn from_iter<I: IntoIterator<Item = &'a Str32>>(iter: I) -> Self {
        String::from_iter(iter.into_iter().map(Str32::as_str))
            .try_into()
            .unwrap()
    }
}

impl iter::FromIterator<Box<str>> for String32 {
    fn from_iter<I: IntoIterator<Item = Box<str>>>(iter: I) -> Self {
        String::from_iter(iter).try_into().unwrap()
    }
}

impl iter::FromIterator<Box<Str32>> for String32 {
    fn from_iter<I: IntoIterator<Item = Box<Str32>>>(iter: I) -> Self {
        String::from_iter(iter.into_iter().map(Str32::into_boxed_str))
            .try_into()
            .unwrap()
    }
}

impl iter::FromIterator<String> for String32 {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        String::from_iter(iter).try_into().unwrap()
    }
}

impl iter::FromIterator<Self> for String32 {
    fn from_iter<I: IntoIterator<Item = Self>>(iter: I) -> Self {
        String::from_iter(iter.into_iter().map(String::from))
            .try_into()
            .unwrap()
    }
}

impl Hash for String32 {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher);
    }
}

impl PartialEq for String32 {
    fn eq(&self, rhs: &Self) -> bool {
        self.as_str().eq(rhs.as_str())
    }
}

impl TryFrom<String> for String32 {
    type Error = TryFromStringError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        u32::try_from(s.len())
            .map(|_| Self(Vec32::from_vec(s.into_bytes())))
            .map_err(|_| TryFromStringError(()))
    }
}
