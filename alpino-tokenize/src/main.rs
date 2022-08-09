use std::io::stdout;

use clap::{builder::EnumValueParser, App, AppSettings, Arg, SubCommand};

mod conll;

mod traits;
use clap_complete::{generate, Shell};
pub use traits::TokenizeApp;

static DEFAULT_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::DontCollapseArgsInUsage,
    AppSettings::UnifiedHelpMessage,
    AppSettings::SubcommandRequiredElseHelp,
];

fn main() {
    let apps = vec![conll::ConlluApp::app()];

    let cli = App::new("finalfusion")
        .settings(DEFAULT_CLAP_SETTINGS)
        .subcommands(apps)
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generate completion scripts for your shell")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(Arg::with_name("shell").value_parser(EnumValueParser::<Shell>::new())),
        );
    let matches = cli.clone().get_matches();

    match matches.subcommand_name().unwrap() {
        "completions" => {
            let shell = matches
                .subcommand_matches("completions")
                .unwrap()
                .get_one::<Shell>("shell")
                .unwrap();
            write_completion_script(cli, *shell);
        }

        "conllu" => conll::ConlluApp::parse(matches.subcommand_matches("conllu").unwrap()).run(),
        _unknown => unreachable!(),
    }
}

fn write_completion_script(mut cli: App, shell: Shell) {
    generate(shell, &mut cli, "alpino-tokenize", &mut stdout());
}
