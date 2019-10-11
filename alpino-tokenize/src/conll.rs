use std::io::{BufRead, BufWriter};
use std::iter::FromIterator;

use alpino_tokenizer::tokenize;
use clap::{App, Arg, ArgMatches};
use conllx::graph::Sentence;
use conllx::io::{WriteSentence, Writer};
use conllx::token::{Features, TokenBuilder};
use lazy_static::lazy_static;
use regex::Regex;
use stdinout::{Input, OrExit, Output};

use crate::TokenizeApp;

// Option constants
static IDENTIFIERS: &str = "IDENTIFIERS";
static WIKIPEDIA: &str = "WIKIPEDIA";

// Argument constants
static INPUT: &str = "INPUT";
static OUTPUT: &str = "OUTPUT";

// Expressions
lazy_static! {
    static ref WIKIPEDIA_DOC_EXPR: Regex =
        Regex::new("<doc.+id=\"([^\"]+)\".+title=\"([^\"]+)\"").unwrap();
}

pub struct ConllxApp {
    input_filename: Option<String>,
    output_filename: Option<String>,
    identifiers: bool,
    wikipedia: bool,
}

impl ConllxApp {
    fn tokenize_para(
        &self,
        lines: &[String],
        writer: &mut impl WriteSentence,
        doc_id: Option<&String>,
        para_id: &mut usize,
    ) {
        if lines.is_empty() {
            return;
        }

        let text = lines.join(" ");
        let tokenized = tokenize(&text).or_exit("Cannot tokenize paragraph", 1);

        for (sent_id, sent) in tokenized.into_iter().enumerate() {
            let graph = if self.identifiers {
                let mut features = Features::from_iter(vec![
                    ("sent".to_string(), Some(sent_id.to_string())),
                    ("para".to_string(), Some(para_id.to_string())),
                ]);

                if doc_id.is_some() {
                    features.insert("doc".to_owned(), doc_id.map(ToOwned::to_owned));
                }

                sent.into_iter()
                    .map(|t| TokenBuilder::new(t).features(features.clone()).into())
                    .collect::<Sentence>()
            } else {
                sent.into_iter()
                    .map(|t| TokenBuilder::new(t).into())
                    .collect::<Sentence>()
            };

            writer
                .write_sentence(&graph)
                .or_exit("Cannot write sentence", 1);
        }

        *para_id += 1;
    }
}

impl TokenizeApp for ConllxApp {
    fn app() -> App<'static, 'static> {
        App::new("conllx")
            .about("Tokenize input and output as CoNLL-X")
            .arg(Arg::with_name(INPUT).help("Input corpus").index(1))
            .arg(Arg::with_name(OUTPUT).help("Output CoNLL-X").index(2))
            .arg(
                Arg::with_name(IDENTIFIERS)
                    .short("i")
                    .help("Add paragraph/sentence identifiers"),
            )
            .arg(
                Arg::with_name(WIKIPEDIA)
                    .long("wikipedia")
                    .help("Process wikiextractor output"),
            )
    }

    fn parse(matches: &ArgMatches) -> Self {
        let input_filename = matches.value_of(INPUT).map(ToOwned::to_owned);
        let output_filename = matches.value_of(OUTPUT).map(ToOwned::to_owned);

        let identifiers = matches.is_present(IDENTIFIERS);
        let wikipedia = matches.is_present(WIKIPEDIA);

        ConllxApp {
            input_filename,
            output_filename,
            identifiers,
            wikipedia,
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
        let mut para_id = 0;
        let mut doc_id = None;

        for line in reader.lines() {
            let line = line.or_exit("Cannot read line", 1);

            if line.trim().is_empty() {
                self.tokenize_para(&para, &mut writer, doc_id.as_ref(), &mut para_id);
                para.clear();
            } else if self.wikipedia {
                if line.starts_with("<doc") {
                    match WIKIPEDIA_DOC_EXPR.captures(&line) {
                        Some(captures) => {
                            doc_id = Some(captures.get(1).unwrap().as_str().to_owned())
                        }
                        None => eprintln!("Could not read identifier in doc tag: {}", line),
                    }
                } else if !line.starts_with("</doc") {
                    para.push(line);
                }
            } else {
                para.push(line);
            }
        }

        self.tokenize_para(&para, &mut writer, doc_id.as_ref(), &mut para_id);
    }
}
