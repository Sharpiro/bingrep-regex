# bingrep-regex

A grep-like tool for searching binary files with regex

> [!warning]
> Warning: Usage is unstable and will change

## Build

```sh
cargo build --release
```

## Usage

```sh
bgr --help
```

### Search

```sh
# prints the matches in format `<no>: [<start_inclusive>, <end_exclusive>): <len>`
1: [0x51424, 0x514c5): 161
```

#### Regex

```sh
# find location of species in Pokemon Crystal
bgr '\x01.{31}\x02.{31}\x03.{31}\x04.{31}\x05.{31}\x06' pokecrystal.gbc
```

#### Experimental binary syntax with partial regex support

```sh
# find location of species in Pokemon Crystal
bgr -b '01 2d [31|32].2d' pokecrystal.gbc
```
