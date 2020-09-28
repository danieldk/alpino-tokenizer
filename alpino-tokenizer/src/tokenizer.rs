use std::io;

use thiserror::Error;

/// Tokenizer errors.
#[derive(Debug, Error)]
pub enum TokenizerError {
    #[error("Cannot read tokenizer protobuf: {0}")]
    RadError(#[from] io::Error),

    #[error("Cannot deserialize tokenizer protobuf: {0}")]
    ProtobufDecodeError(#[from] prost::DecodeError),
}

/// Tokenizer trait type.
pub trait Tokenizer {
    /// Sentence-split and tokenize a paragraph of text.
    ///
    /// The paragraph should be on a single line.
    fn tokenize(&self, text: &str) -> Option<Vec<Vec<String>>>;
}
