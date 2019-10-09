use std::borrow::Cow;

use lazy_static::lazy_static;
use regex::Regex;

// This function rewrites enumerations of the form
//
// 1. foo, 2. bar en 3. baz
//
// to
//
// 1# foo, 2# bar en 3# baz
fn add_enumeration_markers(text: &str) -> Cow<str> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(\\s?1)[.](\\s.*?\\W2[.])").unwrap();
    }

    let mut text = RE.replace_all(text, "$1#$2");

    if let text @ Cow::Borrowed(_) = text {
        return text;
    }

    let mut prev = 1;
    let mut next = 2;

    loop {
        let next_expr = Regex::new(&format!("({}#\\s.*?\\W{})[.](\\s)", prev, next))
            .expect("Invalid enumeration expression.");
        let text_after = next_expr.replace_all(&text, "$1#$2");

        if let Cow::Borrowed(_) = text_after {
            break;
        }

        text = Cow::Owned(text_after.into_owned());
        prev += 1;
        next += 1;
    }

    text
}

pub fn preprocess(text: &str) -> Cow<str> {
    add_enumeration_markers(text)
}

#[cfg(test)]
mod tests {
    use super::preprocess;

    #[test]
    fn test_add_enumeration_markers() {
        assert_eq!(
            preprocess("1. boter, 2. kaas en 3. eieren"),
            "1# boter, 2# kaas en 3# eieren"
        );

        assert_eq!(
            preprocess("1. boter, 2. kaas en 3. eieren, 1. foo en 2. bar"),
            "1# boter, 2# kaas en 3# eieren, 1# foo en 2# bar"
        );
    }
}
