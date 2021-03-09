//! A string that is indexed by `u32` instead of `usize`.
//!
//! On 64-bit platforms, `String32` only requires 16 bytes to store the pointer, length, and capacity. `String` by comparison requires 24 bytes, plus padding.

mod str32;
mod string32;

pub use self::string32::String32;
pub use str32::Str32;

/// The error returned when a string conversion requires a buffer larger than `u32::MAX` bytes.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromStringError(());

const _ASSERT_EQ_SIZE: fn() = || {
    const SIZE: usize = std::mem::size_of::<String32>();
    let _: [u8; SIZE - 16];
    let _: [u8; 16 - SIZE];
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
        assert!(std::mem::size_of::<String32>() == 2 * std::mem::size_of::<usize>());
        assert!(std::mem::size_of::<String32>() <= 2 * std::mem::size_of::<String>());
    }
}
