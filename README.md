# urlendec

A command-line URL encoder/decoder.

[![CI](https://github.com/arcaartem/urlendec/actions/workflows/rust.yml/badge.svg)](https://github.com/arcaartem/urlendec/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/urlendec.svg)](https://crates.io/crates/urlendec)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Features

- Encode or decode percent-encoded URLs
- Read from a file or stdin, write to a file or stdout
- Processes input line-by-line
- Single static binary — no runtime dependencies

## Installation

### From crates.io

```sh
cargo install urlendec
```

### From source

```sh
git clone https://github.com/arcaartem/urlendec.git
cd urlendec
cargo build --release
```

The binary is placed at `target/release/urlendec`.

### Pre-built binaries

Download a binary for your platform from the [releases page](https://github.com/arcaartem/urlendec/releases):

- macOS Intel (`x86_64-apple-darwin`)
- macOS Apple Silicon (`aarch64-apple-darwin`)
- Linux musl static binary (`x86_64-unknown-linux-musl`)

## Usage

Encode a string:

```console
$ urlendec -s "Hello, world!"
Hello%2C%20world%21
```

Decode a string:

```console
$ urlendec -d -s "Hello%2C%20world%21"
Hello, world!
```

Encode from stdin:

```console
$ echo "Hello, world!" | urlendec
Hello%2C%20world%21
```

Encode a file and write to another file:

```sh
urlendec -i input.txt -o output.txt
```

Decode a file to stdout:

```sh
urlendec -d -i encoded.txt
```

## Options

| Flag | Description |
|------|-------------|
| `-d`, `--decode` | Decode input instead of encoding (default is encode) |
| `-s`, `--string <STRING>` | Encode/decode a literal string (mutually exclusive with `-i`) |
| `-i`, `--input-file <PATH>` | Read line-by-line from a file (default: `-` = stdin) |
| `-o`, `--output-file <PATH>` | Write output to a file (default: `-` = stdout) |
| `-h`, `--help` | Print help information |
| `-V`, `--version` | Print version information |

## Building from source

```sh
cargo build --release   # release build
cargo test              # run all tests
cargo clippy            # lint
```

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for setup instructions, style guidelines, and the PR process.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
