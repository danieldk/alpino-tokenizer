use std::os::raw::c_int;

use widestring::WideChar;

extern "C" {
    pub fn new_t_accepts(input: *const WideChar, out: *mut *mut WideChar, max: *mut c_int)
        -> c_int;
}

#[cfg(test)]
mod tests {
    use core::ffi::c_void;
    use std::mem;
    use std::os::raw::c_int;

    use libc::{free, malloc};
    use widestring::{WideCString, WideChar};

    use super::new_t_accepts;

    #[test]
    fn test_new_t_accepts() {
        let input = WideCString::from_str("Dit is een test. En nog een zin.").unwrap();
        let mut output =
            unsafe { malloc((input.len() + 1) * mem::size_of::<WideChar>()) } as *mut WideChar;
        let mut max = (input.len() + 1) as c_int;
        assert_eq!(
            unsafe { new_t_accepts(input.as_ptr(), &mut output, &mut max as *mut c_int) },
            1
        );

        let output_str = unsafe { WideCString::from_ptr_with_nul(output, max as usize) }.unwrap();
        assert_eq!(
            output_str.to_string_lossy(),
            "Dit is een test .\nEn nog een zin ."
        );
        unsafe { free(output as *mut c_void) };
    }
}
