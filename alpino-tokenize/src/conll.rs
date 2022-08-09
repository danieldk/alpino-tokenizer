use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

use alpino_tokenizer::{AlpinoTokenizer, Tokenizer};
use clap::{App, Arg, ArgMatches};
use conllu::io::{WriteSentence, Writer};
use lazy_static::lazy_static;
use regex::Regex;
use stdinout::{Input, OrExit, Output};
use udgraph::graph::{Comment, Sentence};
use udgraph::token::TokenBuilder;

use crate::TokenizeApp;

// Option constants
static IDENTIFIERS: &str = "IDENTIFIERS";
static WIKIPEDIA: &str = "WIKIPEDIA";

// Argument constants
static INPUT: &str = "INPUT";
static OUTPUT: &str = "OUTPUT";
static PROTOBUF: &str = "PROTOBUF";

// Expressions
lazy_static! {
    static ref WIKIPEDIA_DOC_EXPR: Regex =
        Regex::new("<doc.+id=\"([^\"]+)\".+title=\"([^\"]+)\"").unwrap();
}

pub struct ConlluApp {
    input_filename: Option<String>,
    output_filename: Option<String>,
    protobuf_filename: String,
    identifiers: bool,
    wikipedia: bool,
}

impl ConlluApp {
    fn tokenize_para(
        &self,
        tokenizer: &AlpinoTokenizer,
        lines: &[String],
        writer: &mut impl WriteSentence,
        doc_id: Option<&String>,
        doc_title: Option<&String>,
        para_id: &mut usize,
    ) {
        if lines.is_empty() {
            return;
        }

        let text = lines.join(" ");
        let tokenized = tokenizer
            .tokenize(&text)
            .or_exit("Cannot tokenize paragraph", 1);

        for (sent_id, sent) in tokenized.into_iter().enumerate() {
            let mut graph = sent
                .into_iter()
                .map(|t| TokenBuilder::new(t).into())
                .collect::<Sentence>();

            if self.identifiers {
                if let Some(doc_title) = doc_title {
                    graph.comments_mut().push(Comment::AttrVal {
                        attr: "title".to_string(),
                        val: doc_title.clone(),
                    });
                }

                if let Some(doc_id) = doc_id {
                    graph.comments_mut().push(Comment::AttrVal {
                        attr: "sent_id".to_string(),
                        val: format!("d.{}.p.{}.s.{}", doc_id, para_id, sent_id),
                    })
                } else {
                    graph.comments_mut().push(Comment::AttrVal {
                        attr: "sent_id".to_string(),
                        val: format!("p.{}.s.{}", para_id, sent_id),
                    })
                };
            }

            writer
                .write_sentence(&graph)
                .or_exit("Cannot write sentence", 1);
        }

        *para_id += 1;
    }
}

impl TokenizeApp for ConlluApp {
    fn app() -> App<'static, 'static> {
        App::new("conllu")
            .about("Tokenize input and output as CoNLL-X")
            .arg(
                Arg::with_name(PROTOBUF)
                    .help("Tokenizer protobuf")
                    .required(true)
                    .index(1),
            )
            .arg(Arg::with_name(INPUT).help("Input corpus").index(2))
            .arg(Arg::with_name(OUTPUT).help("Output CoNLL-X").index(3))
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
        let protobuf_filename = matches
            .value_of(PROTOBUF)
            .expect("Protobuf filename must be specified")
            .to_owned();

        let identifiers = matches.is_present(IDENTIFIERS);
        let wikipedia = matches.is_present(WIKIPEDIA);

        ConlluApp {
            input_filename,
            output_filename,
            protobuf_filename,
            identifiers,
            wikipedia,
        }
    }

    fn run(&self) {
        let protobuf = BufReader::new(
            File::open(&self.protobuf_filename)
                .or_exit("Cannot open tokenizer protobuf definition", 1),
        );
        let tokenizer =
            AlpinoTokenizer::from_buf_read(protobuf).or_exit("Cannot load tokenizer", 1);

        let input = Input::from(self.input_filename.as_ref());
        let reader = input.buf_read().or_exit("Cannot open input", 1);

        let output = Output::from(self.output_filename.as_ref());
        let mut writer = Writer::new(BufWriter::new(
            output.write().or_exit("Cannot open output", 1),
        ));

        let mut para = vec![];
        let mut para_id = 0;
        let mut doc_id = None;
        let mut doc_title = None;

        for line in reader.lines() {
            let line = line.or_exit("Cannot read line", 1);

            if line.trim().is_empty() {
                self.tokenize_para(
                    &tokenizer,
                    &para,
                    &mut writer,
                    doc_id.as_ref(),
                    doc_title.as_ref(),
                    &mut para_id,
                );
                para.clear();
            } else if self.wikipedia {
                if line.starts_with("<doc") {
                    match WIKIPEDIA_DOC_EXPR.captures(&line) {
                        Some(captures) => {
                            doc_id = Some(captures.get(1).unwrap().as_str().to_owned());
                            doc_title = Some(captures.get(2).unwrap().as_str().to_owned());
                        }
                        None => eprintln!("Could not read identifier in doc tag: {}", line),
                    }

                    para_id = 0;
                } else if !line.starts_with("</doc") {
                    para.push(line);
                }
            } else {
                para.push(line);
            }
        }

        self.tokenize_para(
            &tokenizer,
            &para,
            &mut writer,
            doc_id.as_ref(),
            doc_title.as_ref(),
            &mut para_id,
        );
    }
}
