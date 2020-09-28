use std::io::BufRead;

use crate::postproc::postprocess;
use crate::preproc::preprocess;

use crate::tokenizer::Tokenizer;
use crate::util::str_to_tokens;
use crate::{FiniteStateTokenizer, TokenizerError};

/// Alpino tokenizer and sentence splitter.
pub struct AlpinoTokenizer {
    inner: FiniteStateTokenizer,
}

impl AlpinoTokenizer {
    pub fn from_buf_read<R>(read: R) -> Result<Self, TokenizerError>
    where
        R: BufRead,
    {
        Ok(AlpinoTokenizer {
            inner: FiniteStateTokenizer::from_buf_read(read)?,
        })
    }
}

impl Tokenizer for AlpinoTokenizer {
    fn tokenize(&self, text: &str) -> Option<Vec<Vec<String>>> {
        let tokenized = preprocess(text);
        let tokenized = self.inner.tokenize_raw(tokenized.chars())?;
        let tokenized = postprocess(&tokenized);
        Some(str_to_tokens(&tokenized))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;

    use super::AlpinoTokenizer;
    use crate::util::str_to_tokens;
    use crate::Tokenizer;

    #[test]
    fn test_tokenize() {
        let read = BufReader::new(File::open("testdata/toy.proto").unwrap());
        let tokenizer = AlpinoTokenizer::from_buf_read(read).unwrap();
        assert_eq!(
            tokenizer
                .tokenize("Dit is een zin. En dit is nog een zin...")
                .unwrap(),
            str_to_tokens("Dit is een zin .\nEn dit is nog een zin ...")
        );
    }
}
