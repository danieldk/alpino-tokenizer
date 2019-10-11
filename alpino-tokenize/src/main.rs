use clap::{App, AppSettings, Arg, Shell, SubCommand};

mod conll;

mod traits;
pub use traits::TokenizeApp;

static DEFAULT_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::DontCollapseArgsInUsage,
    AppSettings::UnifiedHelpMessage,
    AppSettings::SubcommandRequiredElseHelp,
];

fn main() {
    let apps = vec![conll::ConllxApp::app()];

    let cli = App::new("finalfusion")
        .settings(DEFAULT_CLAP_SETTINGS)
        .subcommands(apps)
        .subcommand(
            SubCommand::with_name("completions")
                .about("Generate completion scripts for your shell")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(Arg::with_name("shell").possible_values(&Shell::variants())),
        );
    let matches = cli.clone().get_matches();

    match matches.subcommand_name().unwrap() {
        "conllx" => conll::ConllxApp::parse(matches.subcommand_matches("conllx").unwrap()).run(),
        _unknown => unreachable!(),
    }
}
