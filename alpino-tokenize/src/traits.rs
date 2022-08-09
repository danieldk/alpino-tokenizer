use clap::{App, ArgMatches};

pub trait TokenizeApp {
    fn app() -> App<'static>;

    fn parse(matches: &ArgMatches) -> Self;

    fn run(&self);
}
