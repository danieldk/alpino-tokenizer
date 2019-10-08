use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

// ik ga -zoals gezegd- naar huis -> ik ga - zoals gezegd - naar huis
// but, 'huis- tuin- en keuken' should stay as-is
fn fix_dashes(text: &str) -> Cow<str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(" -([^ ][^-]*[^ ])- ").unwrap();
    }

    RE.replace_all(text.as_ref(), |captures: &Captures| {
        let m = captures.get(0).unwrap();
        let left = &text[..m.start()];
        let right = &text[m.end()..];

        if left.ends_with("en")
            || left.ends_with("of")
            || right.starts_with("en")
            || right.starts_with("of")
        {
            Cow::Borrowed(&text[m.start()..m.end()])
        } else {
            let m = captures.get(1).unwrap();
            Cow::Owned(format!(" - {} - ", &text[m.start()..m.end()]))
        }
    })
}

// # AMSTERDAM - ... -> AMSTERDAM -\n...
fn fix_news_article_opening(text: &str) -> Cow<str> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new("(?:^|\n)([[:upper:]]{2}[[:upper:]() /,0-9.-]* -+) ").unwrap();
    }

    RE.replace_all(text.as_ref(), "$1\n")
}

// ( buiten)gewoon -> (buiten)gewoon
fn fix_parens(text: &str) -> Cow<str> {
    lazy_static! {
        static ref RE: Regex = Regex::new("[(] ([[:lower:][:upper:]]+[)])").unwrap();
    }

    RE.replace_all(text.as_ref(), "($1")
}

// # ' top'-vorm -> 'top'-vorm
fn fix_quotes(text: &str) -> Cow<str> {
    lazy_static! {
        static ref RE: Regex = Regex::new("([`'\"]) ([[:upper:][:lower:]]+[`'\"]-)").unwrap();
    }

    RE.replace_all(text.as_ref(), "$1$2")
}

pub fn post_process(text: &str) -> String {
    let text = fix_quotes(text);
    let text = fix_parens(&text);
    let text = fix_news_article_opening(&text);
    let text = fix_dashes(&text);

    text.into_owned()
}

#[cfg(test)]
mod tests {
    use super::post_process;

    #[test]
    fn test_fix_dashes() {
        assert_eq!(
            post_process("huis- tuin- en keuken"),
            "huis- tuin- en keuken"
        );

        assert_eq!(
            post_process("ik ga -zoals gezegd- naar huis"),
            "ik ga - zoals gezegd - naar huis"
        );
    }

    #[test]
    fn test_new_article_opening() {
        assert_eq!(
            post_process("AMSTERDAM - De hoofdstad van Nederland"),
            "AMSTERDAM -\nDe hoofdstad van Nederland"
        );
    }

    #[test]
    fn test_fix_parens() {
        assert_eq!(
            post_process("Dat is ( buiten)gewoon snel ."),
            "Dat is (buiten)gewoon snel ."
        );
    }

    #[test]
    fn test_fix_quotes() {
        assert_eq!(
            post_process("Hij is in ' top'-vorm ."),
            "Hij is in 'top'-vorm ."
        );

        assert_eq!(
            post_process("Hij is in ` top`-vorm ."),
            "Hij is in `top`-vorm ."
        );

        assert_eq!(
            post_process("Hij is in \" top\"-vorm ."),
            "Hij is in \"top\"-vorm ."
        );
    }
}
