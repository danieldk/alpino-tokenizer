use std::mem;
use std::ops::Deref;
use std::str;

use static_assertions::{assert_eq_size, const_assert};

// Long strings are stored in Box<String>, which is one machine
// word. However, since the SmallString enum needs a tag, it
// is two machine words with padding. So, set the maximum small
// string length to two boxes, minus one byte for the length
// and one byte for the tag.
const SMALL_STR_LEN: usize = (2 * mem::size_of::<Box<String>>()) - 2;

/// Immutable string type optimized for short strings.
///
/// This string type uses the following amount of memory:
///
/// 1. Strings shorter than *(2usize) - 2*: *2usize* on the stack;
/// 2. otherwise: *2usize* in the stack, *String* on the heap.
///
/// The string type is immutable to simplify the implementation.
#[derive(Clone, Debug, PartialEq)]
pub enum SmallString {
    /// Small string representation.
    Array { data: [u8; SMALL_STR_LEN], len: u8 },

    /// Long string representation.
    String(Box<String>),
}

// SmallString should be as large as to boxes/machine words.
assert_eq_size!(SmallString, (Box<String>, Box<String>));

// We should be able to represent the length of the small string.
const_assert!(SMALL_STR_LEN <= std::u8::MAX as usize);

impl Deref for SmallString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            SmallString::Array { data, len } => unsafe {
                // This is safe, since the string data was copied from &str or String.
                str::from_utf8_unchecked(&data[..*len as usize])
            },
            SmallString::String(s) => &s,
        }
    }
}

impl From<String> for SmallString {
    fn from(s: String) -> Self {
        if s.len() > SMALL_STR_LEN {
            SmallString::String(Box::new(s))
        } else {
            short_str_to_small_string(&s)
        }
    }
}

impl From<&str> for SmallString {
    fn from(s: &str) -> Self {
        if s.len() > SMALL_STR_LEN {
            SmallString::String(Box::new(s.to_owned()))
        } else {
            short_str_to_small_string(s)
        }
    }
}

fn short_str_to_small_string(s: &str) -> SmallString {
    assert!(s.len() <= SMALL_STR_LEN);

    let mut data = [0; SMALL_STR_LEN];
    data[..s.len()].copy_from_slice(s.as_bytes());
    SmallString::Array {
        data,
        len: s.len() as u8,
    }
}

#[cfg(test)]
mod tests {
    use crate::small_string::SmallString;

    const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    #[test]
    fn small_string_from_str() {
        // This would fail on < 32-bit machines.
        let small = SmallString::from("hello");
        assert!(matches!(small, SmallString::Array {..}));
        assert_eq!(&*small, "hello");

        let large = SmallString::from(LOREM_IPSUM);
        assert!(matches!(large, SmallString::String(_)));
        assert_eq!(&*large, LOREM_IPSUM);
    }

    #[test]
    fn small_string_from_string() {
        // This would fail on < 32-bit machines.
        let small = SmallString::from("hello".to_string());
        assert!(matches!(small, SmallString::Array {..}));
        assert_eq!(&*small, "hello");

        let large = SmallString::from(LOREM_IPSUM.to_string());
        assert!(matches!(large, SmallString::String(_)));
        assert_eq!(&*large, LOREM_IPSUM);
    }
}
