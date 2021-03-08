use std::borrow::{Borrow, BorrowMut};
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

use mediumvec::Vec32;
use usize_cast::IntoUsize;

use super::util;
use super::Str32;
use super::TryFromStringError;

/// A string that is indexed by `u32` instead of `usize`.
///
/// On 32-bit platforms, `String32` behaves similarly to a `String`.
///
/// On 64-bit platforms, `String32` has a smaller memory footprint than `String` struct, but with a maximum capacity of `u32::MAX` instead of `usize::MAX`.
#[derive(Clone, Debug, Default, PartialOrd, Eq, Ord)]
#[repr(transparent)]
pub struct String32 {
    vec: Vec32<u8>,
}

impl String32 {
    /// Creates a new empty `String32`.
    #[must_use]
    pub fn new() -> Self {
        Self { vec: Vec32::new() }
    }

    /// Returns the length of this `String32` in bytes.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn len(&self) -> u32 {
        self.vec.len().try_into().unwrap()
    }

    /// Returns the capacity of this `String32` in bytes.
    #[must_use]
    pub fn capacity(&self) -> u32 {
        self.vec.capacity()
    }

    /// Push a `char` to the end of this `String32`.
    ///
    /// # Panics
    ///
    /// Panics if the resulting string would require more than `u32::MAX` bytes.
    pub fn push(&mut self, ch: char) {
        let mut s = String::from(self.take());
        s.push(ch);
        *self = s.try_into().unwrap();
    }

    /// Pop a `char` from the end of this `String32`.
    pub fn pop(&mut self) -> Option<char> {
        let mut s = String::from(self.take());
        let c = s.pop();
        *self = s.try_into().unwrap();
        c
    }

    /// Return the `char` at a given byte index.
    pub fn remove(&mut self, idx: u32) -> char {
        let mut s = String::from(self.take());
        let c = s.remove(idx.into_usize());
        *self = s.try_into().unwrap();
        c
    }

    /// Insert a `char` at a given byte index.
    ///
    /// # Panics
    ///
    /// Panics if the resulting string would require more than `u32::MAX` bytes.
    pub fn insert(&mut self, idx: u32, ch: char) {
        let mut s = String::from(self.take());
        s.insert(idx.into_usize(), ch);
        *self = s.try_into().unwrap();
    }

    /// Insert a `&str` at the given byte index.
    ///
    /// # Panics
    ///
    /// Panics if the resulting string would require more than `u32::MAX` bytes.
    pub fn insert_str(&mut self, idx: u32, string: &str) {
        let mut s = String::from(self.take());
        s.insert_str(idx.into_usize(), string);
        *self = s.try_into().unwrap();
    }

    /// Creates a new empty `String32` with given capacity.
    #[must_use]
    pub fn with_capacity(cap: u32) -> Self {
        Self {
            vec: Vec32::with_capacity(cap),
        }
    }

    /// Converts a `String32` into a vector of bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.vec.into_vec()
    }

    /// Returns a string slice encompassing the entire `String32`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        unsafe {
            // safety: we never store a non-utf8 Vec32<u8> in a String32
            util::str_from_utf8_unchecked(&self.vec)
        }
    }

    /// Returns a *mutable* string slice encompassing the entire `String32`.
    #[must_use]
    pub fn as_mut_str(&mut self) -> &mut str {
        unsafe {
            // safety: we never store a non-utf8 Vec32<u8> in a String32
            util::str_from_utf8_unchecked_mut(&mut self.vec)
        }
    }

    /// Append a string slice to the end of this `String32`.
    ///
    /// # Panics
    ///
    /// Panics if the resulting string would require more than `u32::MAX` bytes.
    pub fn push_str(&mut self, string: &str) {
        let mut vec = self.take().vec.into_vec();
        vec.extend_from_slice(string.as_bytes());
        self.vec = Vec32::from_vec(vec);
    }

    /// Reserve space for additional bytes.
    pub fn reserve(&mut self, additional: u32) {
        self.vec.reserve(additional)
    }

    /// Reserve space for an exact number of bytes.
    pub fn reserve_exact(&mut self, additional: u32) {
        self.vec.reserve_exact(additional)
    }

    /// Shrink the capacity of this `String32` to match its length.
    pub fn shrink_to_fit(&mut self) {
        let mut vec = self.take().into_bytes();
        vec.shrink_to_fit();
        self.vec = Vec32::from_vec(vec);
    }

    /// Return a byte slice of the `String32`'s contents.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.vec
    }

    /// Shortens this `String32` to the specified length.
    pub fn truncate(&mut self, new_len: u32) {
        let mut s = String::from(self.take());
        s.truncate(new_len.into_usize());
        *self = s.try_into().unwrap();
    }

    /// Return whether the `String32` is an empty string.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Splits the `String32` into two at the given byte index.
    pub fn split_off(&mut self, at: u32) -> Self {
        let mut s = String::from(self.take());
        let other = s.split_off(at.into_usize());
        *self = s.try_into().unwrap();
        other.try_into().unwrap()
    }

    /// Truncates the `String32` into an empty string.
    pub fn clear(&mut self) {
        self.vec.clear()
    }

    /// Converts a `String32` into a `Box<str>`.
    #[must_use]
    pub fn into_boxed_str(self) -> Box<str> {
        String::from(self).into_boxed_str()
    }

    pub fn make_ascii_lowercase(&mut self) {
        let mut s = String::from(self.take());
        s.make_ascii_lowercase();
        *self = s.try_into().unwrap();
    }

    pub fn make_ascii_uppercase(&mut self) {
        let mut s = String::from(self.take());
        s.make_ascii_uppercase();
        *self = s.try_into().unwrap();
    }

    #[must_use]
    fn take(&mut self) -> Self {
        std::mem::take(self)
    }
}

impl AsRef<Str32> for String32 {
    fn as_ref(&self) -> &Str32 {
        &*self
    }
}

impl AsRef<[u8]> for String32 {
    fn as_ref(&self) -> &[u8] {
        &self.vec
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

impl Deref for String32 {
    type Target = Str32;

    fn deref(&self) -> &Str32 {
        unsafe {
            // safety: relies on `&Str32` and `&[u8]` having the same layout. (todo: is there a better way?)
            &*(self.as_str().as_bytes() as *const [u8] as *const Str32)
        }
    }
}

impl DerefMut for String32 {
    fn deref_mut(&mut self) -> &mut Str32 {
        unsafe {
            // safety: relies on `&mut Str32` and `&mut [u8]` having the same layout. (todo: is there a better way?)
            &mut *(self.as_mut_str().as_bytes_mut() as *mut [u8] as *mut Str32)
        }
    }
}

impl fmt::Display for String32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl FromIterator<char> for String32 {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        String::from_iter(iter).try_into().unwrap()
    }
}

impl Hash for String32 {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_str().hash(hasher);
    }
}

impl PartialEq for String32 {
    fn eq(&self, rhs: &Self) -> bool {
        self.deref().eq(rhs)
    }
}

impl From<&Str32> for String32 {
    #[inline]
    fn from(s: &Str32) -> Self {
        s.to_owned()
    }
}

impl From<String32> for String {
    fn from(s: String32) -> Self {
        unsafe {
            // safety: we never store a non-utf8 Vec32<u8> in a String32
            util::string_from_utf8_unchecked(s.vec.into_vec())
        }
    }
}

impl From<String32> for Vec<u8> {
    fn from(s: String32) -> Self {
        s.vec.into_vec()
    }
}

impl TryFrom<String> for String32 {
    type Error = TryFromStringError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let s = s.into_bytes();
        match u32::try_from(s.len()) {
            Ok(_) => Ok(Self {
                vec: Vec32::from_vec(s),
            }),
            Err(_) => Err(TryFromStringError(())),
        }
    }
}