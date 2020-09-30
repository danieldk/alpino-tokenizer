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
//! use std::fs::File;
//! use std::io::BufReader;
//!
//! use alpino_tokenizer::{AlpinoTokenizer, Tokenizer};
//!
//! let read = BufReader::new(File::open("testdata/toy.proto").unwrap());
//! let tokenizer = AlpinoTokenizer::from_buf_read(read).unwrap();
//!
//! assert_eq!(
//!   tokenizer.tokenize("Groningen is een Hanzestad. Groningen heeft veel bezienswaardigheden.").unwrap(),
//!   vec![vec!["Groningen", "is", "een", "Hanzestad", "."],
//!        vec!["Groningen", "heeft", "veel", "bezienswaardigheden", "."]]);
//! ```

mod alpino;
pub use alpino::AlpinoTokenizer;

mod fst;
pub use fst::FiniteStateTokenizer;

mod preproc;

mod postproc;

mod small_string;

mod tokenizer;
pub use tokenizer::{Tokenizer, TokenizerError};

mod util;
