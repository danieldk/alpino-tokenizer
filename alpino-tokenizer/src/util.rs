pub fn str_to_tokens(tokenized: &str) -> Vec<Vec<String>> {
    tokenized
        .split('\n')
        .map(|sent| sent.split(' ').map(ToOwned::to_owned).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
