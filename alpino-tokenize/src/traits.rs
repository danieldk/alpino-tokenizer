use clap::{App, ArgMatches};

pub trait TokenizeApp {
    fn app() -> App<'static, 'static>;

    fn parse(matches: &ArgMatches) -> Self;

    fn run(&self);
}
