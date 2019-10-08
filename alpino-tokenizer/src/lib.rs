use std::mem;
use std::os::raw::{c_int, c_void};
use std::ptr;

use widestring::WideCString;

#[derive(Clone, Copy, Debug)]
pub enum TokenizeError {
    /// Could not allocate memory for tokenization output.
    AllocationError,

    /// The transducer returned a non-terminated string.
    NoStringTerminator,

    /// The string is not in the input language of the transducer.
    NotInInputLanguage,
}

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

fn c_tokenize(text: &str) -> Result<String, TokenizeError> {
    let input = WideCString::from_str(text).unwrap();

    let mut output = PtrBox::alloc_array(input.len() * 2);

    let mut output_len = input.len() as c_int * 2;
    match unsafe {
        alpino_tokenizer_sys::new_t_accepts(
            input.as_ptr(),
            &mut output.as_mut_ptr(),
            &mut output_len as *mut c_int,
        )
    } {
        0 => Err(TokenizeError::NotInInputLanguage),
        1 => Ok(()),
        2 => Err(TokenizeError::AllocationError),
        _ => unreachable!(),
    }?;

    let output_str =
        unsafe { WideCString::from_ptr_with_nul(output.as_mut_ptr(), output_len as usize) }
            .map_err(|_| TokenizeError::NoStringTerminator)?;

    Ok(output_str.to_string_lossy())
}

/// Tokenize a paragraph of text.
///
/// The paragraph should be on a single line.
pub fn tokenize(text: &str) -> Result<Vec<Vec<String>>, TokenizeError> {
    let tokenized = c_tokenize(text)?;
    Ok(tokenized
        .split("\n")
        .map(|sent| sent.split(" ").map(ToOwned::to_owned).collect::<Vec<_>>())
        .collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::{c_tokenize, tokenize};

    fn str_to_tokens(tokenized: &str) -> Vec<Vec<String>> {
        tokenized
            .split("\n")
            .map(|sent| sent.split(" ").map(ToOwned::to_owned).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_c_tokenize() {
        assert_eq!(
            c_tokenize("Dit is een zin. En dit is nog een zin.").unwrap(),
            "Dit is een zin .\nEn dit is nog een zin ."
        );
    }

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("Dit is een zin. En dit is nog een zin.").unwrap(),
            str_to_tokens("Dit is een zin .\nEn dit is nog een zin .")
        );
    }
}
