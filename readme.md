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

Prints each match start address on new line

#### Regex

```sh
# find location of species in Pokemon Crystal
bgr '\x01.{31}\x02.{31}\x03.{31}\x04.{31}\x05.{31}\x06' pokecrystal.gbc
```

#### Context

More details about matches can be displayed with the `--context` or `-c` option starting with value `0`.

```sh
bgr -c 0 '\x01.{31}\x02.{31}\x03.{31}\x04.{31}\x05.{31}\x06' pokecrystal.gbc
```

#### Experimental binary syntax with partial regex support

```sh
# find location of species in Pokemon Crystal
bgr -b '01 2d [31|32].2d' pokecrystal.gbc
```
