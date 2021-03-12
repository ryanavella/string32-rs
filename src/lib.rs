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
    pub fn into_string(self) -> T {
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

    #[test]
    fn simple() {
        const TEXT: &str = include_str!("lib.rs");
        let s1 = String::from(TEXT);
        let mut s2 = String32::new();
        s2.push_str(TEXT);
        assert_eq!(&s1, &s2);
        assert_eq!(s1, s2);
    }
}
