## alpino-tokenizer

This repository provides a Rust wrapper of the
[Alpino](https://www.let.rug.nl/vannoord/alp/Alpino/) tokenizer. You
can use the
[alpino-tokenizer](https://crates.io/crates/alpino-tokenizer) crate in
your Rust programs.

For convenience, an
[alpino-tokenize](https://crates.io/crates/alpino-tokenize)
command-line utility is provided for tokenizing text on from the shell
or in shell scripts.

## Installing the `alpino-tokenize` command-line utility

### cargo

The `alpino-tokenize` utility can be installed with
[cargo](https://rustup.rs/):

```shell
$ cargo install alpino-tokenize
```

### Nix

This repository is also a Nix flake. If you use a Nix version that
supports flakes, you can start a shell with `alpino-tokenize` as
follows:

```
$ nix shell github:danieldk/alpino-tokenizer
```

## License

Copyright 2019-2020 Daniël de Kok

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
