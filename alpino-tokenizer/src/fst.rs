use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

use prost_derive::Message;

use crate::small_string::SmallString;
use crate::tokenizer::Tokenizer;
use crate::util::str_to_tokens;
use crate::TokenizerError;

struct Transducer {
    pub transitions: Vec<Transition>,
}

/// Protobuf transition.
///
/// This data type should only be used during deserialization. Actual
/// transitions are better represented using `Transition`, which performs
/// the small string optimization.
#[derive(Clone, PartialEq, Message)]
struct TransitionProto {
    #[prost(uint32, tag = "1")]
    pub symbol: u32,

    #[prost(bool, tag = "2")]
    pub is_last_of_state: bool,

    #[prost(bool, tag = "3")]
    pub is_final_state: bool,

    #[prost(uint32, tag = "4")]
    pub next: u32,

    #[prost(string, tag = "5")]
    pub output: String,

    #[prost(string, tag = "6")]
    pub final_output: String,
}

/// Transition of a finite state transducer.
struct Transition {
    pub symbol: u32,

    pub is_last_of_state: bool,

    pub is_final_state: bool,

    pub next: u32,

    pub output: SmallString,

    pub final_output: SmallString,
}

impl From<TransitionProto> for Transition {
    fn from(trans: TransitionProto) -> Self {
        Self {
            symbol: trans.symbol,
            is_last_of_state: trans.is_last_of_state,
            is_final_state: trans.is_final_state,
            next: trans.next,
            output: trans.output.into(),
            final_output: trans.final_output.into(),
        }
    }
}

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
        let mut slice = &data[..];

        let mut transitions: Vec<Transition> = Vec::new();

        while !slice.is_empty() {
            let transition: TransitionProto = prost::Message::decode_length_delimited(&mut slice)?;
            transitions.push(transition.into());
        }

        let known_symbols = transitions.iter().map(|t| t.symbol).collect();

        Ok(FiniteStateTokenizer {
            known_symbols,
            transducer: Transducer { transitions },
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
        output: &'a str,
        unknown_queue: &'a mut VecDeque<char>,
    ) -> impl Iterator<Item = char> + 'a {
        // We panic on an empty queue, since this implies an incorrect
        // automaton. Perhaps we could validate these when reading the
        // transducer? Seems prohibitively expensive, since we would
        // have to check every possible path?
        output.chars().map(move |c| {
            if c == char::from(2) {
                unknown_queue
                    .pop_front()
                    .expect("Malformed transducer: unknown character queue is empty.")
            } else {
                c
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
