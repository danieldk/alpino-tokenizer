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

## Copyright

(C) 1999-2018

Gertjan van Noord, Gosse Bouma, Rob Malouf, Robbert Prins, Begona Villada, Jan
Daciuk, Tanja Gaustad, Leonoor van der Beek, Geert Kloosterman, Daniel de Kok,
NWO/RUG.

This library is free software; you can redistribute it and/or modify it under
the terms of the GNU Lesser General Public License as published by the Free
Software Foundation; either version 2.1 of the License, or (at your option) any
later version.

This library is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more
details.

You should have received a copy of the GNU Lesser General Public License along
with this library; if not, write to the

Free Software Foundation, Inc.,
51 Franklin Street, Fifth Floor, Boston,
MA 02110-1301 USA
