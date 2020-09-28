use std::collections::HashSet;
use std::io::{self, BufRead};

use thiserror::Error;

use crate::postproc::postprocess;
use crate::preproc::preprocess;
use crate::proto::Transducer;
use crate::util::str_to_tokens;

#[derive(Debug, Error)]
pub enum TokenizerError {
    #[error("Cannot read tokenizer protobuf: {0}")]
    RadError(#[from] io::Error),

    #[error("Cannot deserialize tokenizer protobuf: {0}")]
    ProtobufDecodeError(#[from] prost::DecodeError),
}

pub struct Tokenizer {
    known_symbols: HashSet<u32>,
    transducer: Transducer,
}

impl Tokenizer {
    pub fn from_buf_read<R>(mut read: R) -> Result<Self, TokenizerError>
    where
        R: BufRead,
    {
        let mut data = Vec::new();
        read.read_to_end(&mut data)?;

        let transducer: Transducer = prost::Message::decode(&*data)?;
        let known_symbols = transducer.transitions.iter().map(|t| t.symbol).collect();

        Ok(Tokenizer {
            known_symbols,
            transducer,
        })
    }

    /// Sentence split and tokenize a paragraph of text.
    ///
    /// The paragraph should be on a single line.
    pub fn tokenize(&self, text: &str) -> Option<Vec<Vec<String>>> {
        let tokenized = preprocess(text);
        let tokenized = self.tokenize_raw(tokenized.chars())?;
        let tokenized = postprocess(&tokenized);
        Some(str_to_tokens(&tokenized))
    }

    pub fn tokenize_raw<I>(&self, chars: I) -> Option<String>
    where
        I: IntoIterator<Item = char>,
    {
        let mut output = String::new();

        let mut trans_offset = 1;
        let mut transition = &self.transducer.transitions[trans_offset];
        for ch in chars {
            trans_offset = transition.next as usize;
            transition = &self.transducer.transitions[trans_offset];
            let symbol = ch as u32;

            if transition.symbol != 1 || self.known_symbols.contains(&symbol) {
                if transition.symbol == 2 && !self.known_symbols.contains(&symbol) {
                    output.extend(transition.output.chars().map(|out_ch| {
                        if out_ch == char::from(2) {
                            ch
                        } else {
                            out_ch
                        }
                    }))
                } else {
                    while !transition.is_last_of_state && symbol > transition.symbol {
                        trans_offset += 1;
                        transition = &self.transducer.transitions[trans_offset];
                    }

                    if transition.symbol != symbol {
                        return None;
                    } else {
                        output.push_str(&transition.output);
                    }
                }
            }
        }

        output.push_str(&transition.final_output);

        Some(output)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;

    use super::Tokenizer;
    use crate::util::str_to_tokens;

    #[test]
    fn test_tokenize() {
        let read = BufReader::new(File::open("testdata/toy.proto").unwrap());
        let tokenizer = Tokenizer::from_buf_read(read).unwrap();
        assert_eq!(
            tokenizer
                .tokenize("Dit is een zin. En dit is nog een zin...")
                .unwrap(),
            str_to_tokens("Dit is een zin .\nEn dit is nog een zin ...")
        );
    }
}
