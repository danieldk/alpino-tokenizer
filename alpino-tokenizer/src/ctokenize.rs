use std::error::Error;
use std::fmt;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use widestring::WideCString;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenizeError {
    /// Could not allocate memory for tokenization output.
    AllocationError,

    /// The input contains the NUL character.
    InputContainsNul,

    /// The transducer returned a non-terminated string.
    NoStringTerminator,

    /// The string is not in the input language of the transducer.
    NotInInputLanguage,
}

impl fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use TokenizeError::*;
        match self {
            AllocationError => write!(f, "could not allocate memory for output string")?,
            InputContainsNul => write!(f, "the input string contained a NUL character")?,
            NoStringTerminator => write!(f, "the transducer returned a non-terminated string")?,
            NotInInputLanguage => write!(
                f,
                "the input string is not in the input language of the transducer"
            )?,
        }

        Ok(())
    }
}

impl Error for TokenizeError {}

/// A small pointer wrapper that frees when it goes out of scope.
struct PtrBox<T> {
    inner: *mut T,
}

impl<T> PtrBox<T> {
    fn alloc_array(n_elems: usize) -> Self {
        let inner = unsafe { libc::malloc(n_elems * mem::size_of::<T>()) } as *mut T;
        PtrBox { inner }
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        self.inner
    }
}

impl<T> Drop for PtrBox<T> {
    fn drop(&mut self) {
        unsafe { libc::free(self.inner as *mut c_void) };
        self.inner = ptr::null_mut();
    }
}

pub fn c_tokenize(text: &str) -> Result<String, TokenizeError> {
    let input = match WideCString::from_str(text) {
        Ok(input) => input,
        Err(_) => return Err(TokenizeError::InputContainsNul),
    };

    let mut output_len = input.len() * 2;
    let mut output = PtrBox::alloc_array(output_len);

    match unsafe {
        alpino_tokenizer_sys::new_t_accepts(
            input.as_ptr(),
            &mut output.as_mut_ptr(),
            &mut output_len,
        )
    } {
        0 => Err(TokenizeError::NotInInputLanguage),
        1 => Ok(()),
        2 => Err(TokenizeError::AllocationError),
        _ => unreachable!(),
    }?;

    let output_str = unsafe { WideCString::from_ptr_with_nul(output.as_mut_ptr(), output_len) }
        .map_err(|_| TokenizeError::NoStringTerminator)?;

    Ok(output_str.to_string_lossy())
}

#[cfg(test)]
mod tests {
    use super::{c_tokenize, TokenizeError};

    #[test]
    fn test_c_tokenize() {
        assert_eq!(
            c_tokenize("Dit is een zin. En dit is nog een zin.").unwrap(),
            "Dit is een zin .\nEn dit is nog een zin ."
        );
    }

    #[test]
    fn test_handle_nul_in_input() {
        assert_eq!(
            c_tokenize("Deze string bevat een NUL karakter\0"),
            Err(TokenizeError::InputContainsNul)
        );
    }
}
