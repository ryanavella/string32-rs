//! A string that is indexed by `u32` instead of `usize`.
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

mod charindices;
mod str32;
mod string32;
mod util;

pub use self::string32::String32;
pub use charindices::CharIndices;
pub use str32::Str32;

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
