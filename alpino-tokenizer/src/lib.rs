mod ctokenize;
use ctokenize::c_tokenize;
pub use ctokenize::TokenizeError;

mod preproc;
use preproc::preprocess;

mod postproc;
use postproc::postprocess;

mod util;
pub(crate) use util::str_to_tokens;

/// Tokenize a paragraph of text.
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
