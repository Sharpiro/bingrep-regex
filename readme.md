# bingrep-regex

A grep-like tool for searching binary files with regex

> Warning: Usage is unstable and will change

## Build

```sh
cargo build --release
```

## Usage

```sh
bgr --help
```

Example:

```sh
# find location of species in Pokemon Crystal
bgr '\x01.{31}\x02.{31}\x03.{31}\x04.{31}\x05.{31}\x06' pokecrystal.gbc
```

```sh
# prints the matches in format `[<start>, <end>): <len>`
1: [0x51424, 0x514c5): 161
```
