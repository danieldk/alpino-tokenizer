//! Wrapper for the Alpino tokenizer for Dutch.
//!
//! This crate provides a wrapper around the Alpino tokenizer for
//! Dutch. Since the tokenizer itself is included through the
//! `alpino-tokenizer-sys` crate, this crate can be used without
//! any external dependencies.
//!
//! This crate exposes a single function `tokenize`, that takes a
//! single paragraph as a string and returns a `Vec<Vec<String>>`
//! holding the sentences and tokens. For example:
//!
//! ```
//! use alpino_tokenizer::tokenize;
//!
//! assert_eq!(
//!   tokenize("Groningen is een Hanzestad. Groningen heeft 201.635 inwoners.").unwrap(),
//!   vec![vec!["Groningen", "is", "een", "Hanzestad", "."],
//!        vec!["Groningen", "heeft", "201.635", "inwoners", "."]]);
//! ```

mod ctokenize;
use ctokenize::c_tokenize;
pub use ctokenize::TokenizeError;

mod preproc;
use preproc::preprocess;

mod postproc;
use postproc::postprocess;

mod util;
pub(crate) use util::str_to_tokens;

/// Sentence split and tokenize a paragraph of text.
///
/// The paragraph should be on a single line.
pub fn tokenize(text: &str) -> Result<Vec<Vec<String>>, TokenizeError> {
    let tokenized = preprocess(text);
    let tokenized = c_tokenize(&tokenized)?;
    let tokenized = postprocess(&tokenized);
    Ok(str_to_tokens(&tokenized))
}

#[cfg(test)]
mod tests {
    use super::{str_to_tokens, tokenize};

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("Dit is een zin. En dit is nog een zin.").unwrap(),
            str_to_tokens("Dit is een zin .\nEn dit is nog een zin .")
        );
    }
}
