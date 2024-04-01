# bingrep

A grep-like tool for searching binary files with regex

## Build

```sh
cargo build --release
```

## Usage

```sh
bingrep --help
```

Example:

```sh
# find location of species in Pokemon Crystal
bingrep '\x01.{31}\x02.{31}\x03.{31}\x04.{31}\x05.{31}\x06' pokecrystal.gbc
```

```sh
# prints the matches in format `(start, end)`
Match 1/1: (0x51424, 0x514c5)
```
