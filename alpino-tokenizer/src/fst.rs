use std::collections::{HashSet, VecDeque};
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

        let mut unknown_queue = VecDeque::new();
        let mut trans_offset = 1;
        let mut transition = &self.transducer.transitions[trans_offset];
        for ch in chars {
            trans_offset = transition.next as usize;
            transition = &self.transducer.transitions[trans_offset];
            let symbol = ch as u32;

            if transition.symbol != 1 || self.known_symbols.contains(&symbol) {
                // If the character is unknown and we are in a transition that handles
                // unknown characters, we are done. Otherwise, find a transition matching
                // the character.
                if transition.symbol == 2 && !self.known_symbols.contains(&symbol) {
                    unknown_queue.push_back(ch);
                } else {
                    // Linearly scan the transitions until we have found one that matches
                    // the character.
                    while !transition.is_last_of_state && symbol > transition.symbol {
                        trans_offset += 1;
                        transition = &self.transducer.transitions[trans_offset];
                    }

                    // If the current transition is not a match, the string is not in the
                    // language of the transducer.
                    if transition.symbol != symbol {
                        return None;
                    }
                }
            }

            // Append transition output, replacing unknown characters from
            // the unknown character queue.
            output.extend(Self::replace_output_with_queue(
                &transition.output,
                &mut unknown_queue,
            ));
        }

        // Append final output, replacing unknown characters from the unknown
        // character queue.
        output.extend(Self::replace_output_with_queue(
            &transition.final_output,
            &mut unknown_queue,
        ));

        Some(output)
    }
    fn replace_output_with_queue<'a>(
        output: &'a [u32],
        unknown_queue: &'a mut VecDeque<char>,
    ) -> impl Iterator<Item = char> + 'a {
        // We panic on an empty queue or an invalid character in the
        // output, since this implies an incorrect automaton. Perhaps
        // we should validate these when reading the transducer?
        output.iter().map(move |&c| {
            if c == 2 {
                unknown_queue
                    .pop_front()
                    .expect("Malformed transducer: unknown character queue is empty.")
            } else {
                std::char::from_u32(c)
                    .expect("Malformed transducer: invalid character in transition output")
            }
        })
    }
}

impl Tokenizer for FiniteStateTokenizer {
    fn tokenize(&self, text: &str) -> Option<Vec<Vec<String>>> {
        let tokenized = self.tokenize_raw(text.chars())?;
        Some(str_to_tokens(&tokenized))
    }
}
