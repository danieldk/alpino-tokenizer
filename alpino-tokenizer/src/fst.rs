use std::collections::HashSet;
use std::io::BufRead;

use crate::proto::Transducer;
use crate::tokenizer::Tokenizer;
use crate::util::str_to_tokens;
use crate::TokenizerError;

/// Finite state tokenizer and sentence splitter.
///
/// This type implements a tokenizer based on a finite-state transducer. In
/// principle, it should work with any tokenizing transducer built with the
/// [FSA utilities](https://www.let.rug.nl/~vannoord/Fsa/).
///
/// If Alpino's transducer is used for tokenization, it is strongly recommended
/// to use the `AlpinoTokenizer` data type. `AlpinoTokenizer` applies pre- and
/// post-processing steps that are expected by Alpino's transducer.
pub struct FiniteStateTokenizer {
    known_symbols: HashSet<u32>,
    transducer: Transducer,
}

impl FiniteStateTokenizer {
    pub fn from_buf_read<R>(mut read: R) -> Result<Self, TokenizerError>
    where
        R: BufRead,
    {
        let mut data = Vec::new();
        read.read_to_end(&mut data)?;

        let transducer: Transducer = prost::Message::decode(&*data)?;
        let known_symbols = transducer.transitions.iter().map(|t| t.symbol).collect();

        Ok(FiniteStateTokenizer {
            known_symbols,
            transducer,
        })
    }

    pub(crate) fn tokenize_raw<I>(&self, chars: I) -> Option<String>
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

impl Tokenizer for FiniteStateTokenizer {
    fn tokenize(&self, text: &str) -> Option<Vec<Vec<String>>> {
        let tokenized = self.tokenize_raw(text.chars())?;
        Some(str_to_tokens(&tokenized))
    }
}
