use std::io::{BufRead, BufWriter};

use alpino_tokenizer::tokenize;
use clap::{App, Arg, ArgMatches};
use conllx::graph::Sentence;
use conllx::io::{WriteSentence, Writer};
use conllx::token::TokenBuilder;
use stdinout::{Input, OrExit, Output};

use crate::TokenizeApp;

// Argument constants
static INPUT: &str = "INPUT";
static OUTPUT: &str = "OUTPUT";

pub struct ConllxApp {
    input_filename: Option<String>,
    output_filename: Option<String>,
}

impl TokenizeApp for ConllxApp {
    fn app() -> App<'static, 'static> {
        App::new("conllx")
            .about("Tokenize input and output as CoNLL-X")
            .arg(Arg::with_name(INPUT).help("Input corpus").index(1))
            .arg(Arg::with_name(OUTPUT).help("Output CoNLL-X").index(2))
    }

    fn parse(matches: &ArgMatches) -> Self {
        let input_filename = matches.value_of(INPUT).map(ToOwned::to_owned);
        let output_filename = matches.value_of(OUTPUT).map(ToOwned::to_owned);

        ConllxApp {
            input_filename,
            output_filename,
        }
    }

    fn run(&self) {
        let input = Input::from(self.input_filename.as_ref());
        let reader = input.buf_read().or_exit("Cannot open input", 1);

        let output = Output::from(self.output_filename.as_ref());
        let mut writer = Writer::new(BufWriter::new(
            output.write().or_exit("Cannot open output", 1),
        ));

        let mut para = vec![];
        for line in reader.lines() {
            let line = line.or_exit("Cannot read line", 1);

            if line.trim().is_empty() {
                tokenize_para(&para, &mut writer);
                para.clear();
            } else {
                para.push(line);
            }
        }

        tokenize_para(&para, &mut writer);
    }
}

fn tokenize_para(lines: &[String], writer: &mut impl WriteSentence) {
    if lines.is_empty() {
        return;
    }

    let text = lines.join(" ");
    let tokenized = tokenize(&text).or_exit("Cannot tokenize paragraph", 1);

    for sent in tokenized {
        let graph = sent
            .into_iter()
            .map(|t| TokenBuilder::new(t).into())
            .collect::<Sentence>();
        writer
            .write_sentence(&graph)
            .or_exit("Cannot write sentence", 1);
    }
}
