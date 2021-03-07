#[must_use]
pub unsafe fn str_from_utf8_unchecked(v: &[u8]) -> &str {
    if cfg!(debug_assertions) {
        std::str::from_utf8(v).unwrap()
    } else {
        std::str::from_utf8_unchecked(v)
    }
}

#[must_use]
pub unsafe fn str_from_utf8_unchecked_mut(v: &mut [u8]) -> &mut str {
    if cfg!(debug_assertions) {
        std::str::from_utf8_mut(v).unwrap()
    } else {
        std::str::from_utf8_unchecked_mut(v)
    }
}

#[must_use]
pub unsafe fn string_from_utf8_unchecked(v: Vec<u8>) -> String {
    if cfg!(debug_assertions) {
        String::from_utf8(v).unwrap()
    } else {
        String::from_utf8_unchecked(v)
    }
}
