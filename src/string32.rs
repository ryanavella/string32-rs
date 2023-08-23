use std::borrow::{Borrow, BorrowMut, Cow};
use std::cmp;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::mem;
use std::ops;
use std::string;

use mediumvec::Vec32;
use usize_cast::IntoUsize;

use super::{Str32, TryFromStrError, TryFromStringError};

/// A string that is indexed by `u32` instead of `usize`.
///
/// On 64-bit platforms, `String32` only requires 16 bytes to store the pointer, length, and capacity. [`String`] by comparison requires 24 bytes, plus padding.
#[derive(Clone, Debug, Default, Eq)]
#[repr(transparent)]
pub struct String32(Vec32<u8>);

impl String32 {
    /// Create an empty `String32`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// let s = String32::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self(Vec32::new())
    }

    /// Create an empty `String32` with enough capacity to hold `cap` bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// let mut s = String32::with_capacity(1);
    /// let cap = s.capacity();
    /// s.push('\n');
    /// assert_eq!(cap, s.capacity());
    /// ```
    #[must_use]
    pub fn with_capacity(cap: u32) -> Self {
        Self(Vec32::with_capacity(cap))
    }

    /// Return the capacity of this `String32` in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// let mut s = String32::new();
    /// assert_eq!(0, s.capacity());
    /// s.push('\n');
    /// assert!(s.capacity() > 0);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> u32 {
        self.0.capacity()
    }

    /// A helper to call arbitrary [`String`] methods on a `String32.`
    ///
    /// # Panics
    ///
    /// Panics if the resulting string would require more than [`u32::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// let mut s = String32::new();
    /// s.as_string(|s| s.push_str("test"));
    /// assert_eq!(s, "test");
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// let mut s = String32::new();
    /// s.push('\n');
    /// assert_eq!(s, "\n");
    /// ```
    pub fn push(&mut self, ch: char) {
        self.as_string(|s| s.push(ch));
    }

    /// Append a string slice to the end of this `String32`.
    ///
    /// # Panics
    ///
    /// Panics if the resulting string would require more than [`u32::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// let mut s = String32::new();
    /// s.push_str("test");
    /// assert_eq!(s, "test");
    /// ```
    pub fn push_str<S>(&mut self, s: S)
    where
        S: AsRef<str>,
    {
        self.as_string(|st| st.push_str(s.as_ref()));
    }

    /// Pop a `char` from the end of this `String32`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// # use std::convert::TryFrom;
    /// let mut s = String32::try_from("\n").unwrap();
    /// assert_eq!(s.pop(), Some('\n'));
    /// assert_eq!(s.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<char> {
        self.as_string(String::pop)
    }

    /// Return the `char` at a given byte index.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is not a UTF-8 code point boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// # use std::convert::TryFrom;
    /// let mut s = String32::try_from("abbc").unwrap();
    /// assert_eq!(s.remove(1), 'b');
    /// assert_eq!(s, "abc");
    /// ```
    pub fn remove(&mut self, idx: u32) -> char {
        self.as_string(|s| s.remove(idx.into_usize()))
    }

    /// Insert a `char` at a given byte index.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is not a UTF-8 code point boundary, or if the resulting string would require more than [`u32::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// # use std::convert::TryFrom;
    /// let mut s = String32::try_from("ac").unwrap();
    /// s.insert(1, 'b');
    /// assert_eq!(s, "abc");
    /// ```
    pub fn insert(&mut self, idx: u32, ch: char) {
        self.as_string(|s| s.insert(idx.into_usize(), ch));
    }

    /// Insert a string slice at the given byte index.
    ///
    /// # Panics
    ///
    /// Panics if `idx` is not a UTF-8 code point boundary, or if the resulting string would require more than [`u32::MAX`] bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// # use std::convert::TryFrom;
    /// let mut s = String32::try_from("ad").unwrap();
    /// s.insert_str(1, "bc");
    /// assert_eq!(s, "abcd");
    /// ```
    pub fn insert_str<S>(&mut self, idx: u32, s: S)
    where
        S: AsRef<str>,
    {
        self.as_string(|st| st.insert_str(idx.into_usize(), s.as_ref()));
    }

    /// Reserve space for additional bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// # use std::convert::TryFrom;
    /// let mut s = String32::try_from("abc").unwrap();
    /// s.reserve(10);
    /// println!("{}", s.capacity());
    /// assert!(s.capacity() >= 13);
    /// ```
    pub fn reserve(&mut self, additional: u32) {
        self.0.reserve(additional)
    }

    /// Reserve space for an exact number of bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// let mut s = String32::with_capacity(5);
    /// s.reserve_exact(10);
    /// assert!(s.capacity() >= 10);
    /// ```
    pub fn reserve_exact(&mut self, additional: u32) {
        self.0.reserve_exact(additional)
    }

    /// Shrink the capacity of this `String32` to match its length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// let mut s = String32::with_capacity(10);
    /// s.shrink_to_fit();
    /// assert_eq!(0, s.capacity());
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.as_string(String::shrink_to_fit);
    }

    /// Shortens this `String32` to the specified length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// # use std::convert::TryFrom;
    /// let mut s = String32::try_from("abcde").unwrap();
    /// s.truncate(3);
    /// assert_eq!(s, "abc");
    /// ```
    pub fn truncate(&mut self, new_len: u32) {
        self.as_string(|s| s.truncate(new_len.into_usize()));
    }

    /// Truncates the `String32` into an empty string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// # use std::convert::TryFrom;
    /// let mut s = String32::try_from("abc").unwrap();
    /// s.clear();
    /// assert!(s.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Converts a `String32` into a vector of bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// # use std::convert::TryFrom;
    /// let s = String32::try_from("123").unwrap();
    /// let v = s.into_bytes();
    /// assert_eq!(v, b"123");
    /// ```
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.0.into_vec()
    }

    /// Converts a `String32` into a [`Box<str>`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// # use std::convert::TryFrom;
    /// let s = String32::try_from("123").unwrap();
    /// let b = s.into_boxed_str();
    /// ```
    #[must_use]
    pub fn into_boxed_str(self) -> Box<str> {
        String::from(self).into_boxed_str()
    }

    /// Splits the `String32` into two at the given byte index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out-of-bounds or is not a UTF-8 code point boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// # use string32::String32;
    /// # use std::convert::TryFrom;
    /// let mut s1 = String32::try_from("123abc").unwrap();
    /// let s2 = s1.split_off(3);
    /// assert_eq!("123", s1);
    /// assert_eq!("abc", s2);
    /// ```
    #[must_use = "if you plan to discard the second half, consider using `String32::truncate` instead"]
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
    pub fn from_utf8(v: Vec<u8>) -> Result<Self, string::FromUtf8Error> {
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
    pub fn from_utf16(v: &[u16]) -> Result<Self, string::FromUtf16Error> {
        String::from_utf16(v).map(|s| s.try_into().unwrap())
    }

    /// Lossily decodes a UTF-16 encoded slice into a `String32`.
    ///
    /// # Panics
    ///
    /// Panics if the resulting UTF-8 representation would require more than [`u32::MAX`] bytes.
    #[must_use]
    pub fn from_utf16_lossy(v: &[u16]) -> Self {
        String::from_utf16_lossy(v).try_into().unwrap()
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
        self
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
        self
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
        let ptr = self.0.as_ref() as *const [u8] as *const str as *const Str32;
        unsafe {
            // safety: relies on `&Str32` and `&str` having the same layout
            &*ptr
        }
    }
}

impl ops::DerefMut for String32 {
    fn deref_mut(&mut self) -> &mut Str32 {
        let ptr = self.0.as_mut() as *mut [u8] as *mut str as *mut Str32;
        unsafe {
            // safety: relies on `&mut Str32` and `&mut str` having the same layout
            &mut *ptr
        }
    }
}

impl fmt::Display for String32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Str32 as fmt::Display>::fmt(self, f)
    }
}

impl From<&Str32> for String32 {
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

impl FromIterator<char> for String32 {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        String::from_iter(iter).try_into().unwrap()
    }
}

impl<'a> FromIterator<&'a char> for String32 {
    fn from_iter<I: IntoIterator<Item = &'a char>>(iter: I) -> Self {
        String::from_iter(iter).try_into().unwrap()
    }
}

impl<'a> FromIterator<&'a str> for String32 {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        String::from_iter(iter).try_into().unwrap()
    }
}

impl<'a> FromIterator<&'a Str32> for String32 {
    fn from_iter<I: IntoIterator<Item = &'a Str32>>(iter: I) -> Self {
        String::from_iter(iter.into_iter().map(Str32::as_str))
            .try_into()
            .unwrap()
    }
}

impl FromIterator<Box<str>> for String32 {
    fn from_iter<I: IntoIterator<Item = Box<str>>>(iter: I) -> Self {
        String::from_iter(iter).try_into().unwrap()
    }
}

impl FromIterator<Box<Str32>> for String32 {
    fn from_iter<I: IntoIterator<Item = Box<Str32>>>(iter: I) -> Self {
        String::from_iter(iter.into_iter().map(Str32::into_boxed_str))
            .try_into()
            .unwrap()
    }
}

impl FromIterator<String> for String32 {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        String::from_iter(iter).try_into().unwrap()
    }
}

impl FromIterator<Self> for String32 {
    fn from_iter<I: IntoIterator<Item = Self>>(iter: I) -> Self {
        String::from_iter(iter.into_iter().map(String::from))
            .try_into()
            .unwrap()
    }
}

impl Hash for String32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl Ord for String32 {
    fn cmp(&self, rhs: &Self) -> cmp::Ordering {
        <Str32 as Ord>::cmp(self, rhs)
    }
}

impl PartialEq for String32 {
    fn eq(&self, rhs: &Self) -> bool {
        <Str32 as PartialEq>::eq(self, rhs)
    }
}

impl PartialOrd for String32 {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        <Str32 as PartialOrd>::partial_cmp(self, rhs)
    }
}

macro_rules! impl_cmp {
    ($lhs:ty, $rhs: ty) => {
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            fn eq(&self, rhs: &$rhs) -> bool {
                <[u8] as PartialEq>::eq(self.as_bytes(), rhs.as_bytes())
            }
        }

        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            fn eq(&self, rhs: &$lhs) -> bool {
                <[u8] as PartialEq>::eq(self.as_bytes(), rhs.as_bytes())
            }
        }

        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            fn partial_cmp(&self, rhs: &$rhs) -> Option<cmp::Ordering> {
                <[u8] as PartialOrd>::partial_cmp(self.as_bytes(), rhs.as_bytes())
            }
        }

        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            fn partial_cmp(&self, rhs: &$lhs) -> Option<cmp::Ordering> {
                <[u8] as PartialOrd>::partial_cmp(self.as_bytes(), rhs.as_bytes())
            }
        }
    };
}

impl_cmp!(String32, Str32);
impl_cmp!(String32, &'a Str32);
impl_cmp!(String32, String);
impl_cmp!(String32, str);
impl_cmp!(String32, &'a str);
impl_cmp!(String32, Cow<'a, Str32>);
impl_cmp!(String32, Cow<'a, str>);
impl_cmp!(String32, Box<str>);
impl_cmp!(String32, Box<Str32>);
impl_cmp!(Str32, &'a Str32);
impl_cmp!(Str32, String);
impl_cmp!(Str32, str);
impl_cmp!(Str32, &'a str);
impl_cmp!(Str32, Cow<'a, Str32>);
impl_cmp!(Str32, Cow<'a, str>);
impl_cmp!(Str32, Box<str>);
impl_cmp!(Str32, Box<Str32>);
impl_cmp!(&'a Str32, String);
impl_cmp!(&'a Str32, str);
impl_cmp!(&'a Str32, Cow<'b, Str32>);
impl_cmp!(&'a Str32, Cow<'b, str>);
impl_cmp!(&'a Str32, Box<str>);
impl_cmp!(&'a Str32, Box<Str32>);

impl TryFrom<String> for String32 {
    type Error = TryFromStringError<String>;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match u32::try_from(s.len()) {
            Ok(_) => Ok(Self(Vec32::from_vec(s.into_bytes()))),
            Err(_) => Err(TryFromStringError(s)),
        }
    }
}

impl TryFrom<Box<str>> for String32 {
    type Error = TryFromStringError<Box<str>>;

    fn try_from(s: Box<str>) -> Result<Self, Self::Error> {
        match u32::try_from(s.len()) {
            Ok(_) => Ok(Self(Vec32::from_vec(String::from(s).into_bytes()))),
            Err(_) => Err(TryFromStringError(s)),
        }
    }
}

impl TryFrom<&str> for String32 {
    type Error = TryFromStrError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match u32::try_from(s.len()) {
            Ok(_) => Ok(Self(Vec32::from_vec(s.to_owned().into_bytes()))),
            Err(_) => Err(TryFromStrError(())),
        }
    }
}
