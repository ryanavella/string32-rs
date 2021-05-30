//! A string that is indexed by `u32` instead of `usize`.
//!
//! On 64-bit platforms, `String32` only requires 16 bytes to store the pointer, length, and capacity. `String` by comparison requires 24 bytes, plus padding.
use std::fmt;
use std::mem::{align_of, size_of};

mod str32;
mod string32;

pub use crate::string32::String32;
pub use str32::Str32;

/// The error returned when a `String` conversion to `String32` would require a buffer larger than `u32::MAX` bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryFromStringError<T>(T);

impl<T> TryFromStringError<T> {
    /// Return the string that was unable to be converted into a `String32`.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> fmt::Display for TryFromStringError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "string too large for u32-indexed buffer")
    }
}

/// The error returned when a `&str` conversion to `&Str32` or `String32` would require a buffer larger than `u32::MAX` bytes.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromStrError(());

impl fmt::Display for TryFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "string too large for u32-indexed buffer")
    }
}

macro_rules! comptime_assert_eq {
    ($lhs:expr, $rhs:expr $(,)?) => {
        const _: [(); ($lhs == $rhs) as usize - 1] = [];
    };
}

// Should be true for both 32-bit and 64-bit platforms
comptime_assert_eq!(size_of::<String32>(), 8 + size_of::<usize>());
comptime_assert_eq!(align_of::<String32>(), align_of::<usize>());
comptime_assert_eq!(size_of::<&str>(), size_of::<&Str32>());
comptime_assert_eq!(align_of::<&str>(), align_of::<&Str32>());

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    const TEXT: &str = include_str!("lib.rs");

    #[test]
    fn test_simple() {
        let s1 = String::from(TEXT);
        let mut s2 = String32::new();
        s2.push_str(TEXT);
        assert_eq!(&s1, &s2);
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_complex() {
        let mut s = String32::try_from(TEXT).unwrap();
        s.shrink_to_fit();
        s.push('\n');
        s.reserve_exact(2 * s.len());
        s.reserve(1);
        s.push_str(TEXT);
        s.pop();
        s.remove(s.len() - 1);
        s.remove(0);
        s.insert(s.len() - 1, '\n');
        s.insert(0, '\n');
        s.insert_str(0, TEXT);
        s.truncate(s.len() / 2);
        let mut other = s.split_off(s.len() / 2);
        other.push_str(&s);
        assert!(!other.is_empty());
    }

    #[test]
    fn test_hash() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hash1 = DefaultHasher::new();
        let mut hash2 = DefaultHasher::new();

        let s1 = String::from(TEXT);
        let s2 = String32::try_from(TEXT).unwrap();

        <String as Hash>::hash(&s1, &mut hash1);
        <String32 as Hash>::hash(&s2, &mut hash2);
        <str as Hash>::hash(&s1, &mut hash1);
        <Str32 as Hash>::hash(&s2, &mut hash2);

        assert_eq!(hash1.finish(), hash2.finish());
    }
}
