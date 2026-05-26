# Contributing to urlendec

Thank you for your interest in contributing. Here is everything you need to get started.

## Setting up

1. Install Rust via [rustup](https://rustup.rs/).
2. Clone the repository:

   ```sh
   git clone https://github.com/arcaartem/urlendec.git
   cd urlendec
   ```

3. Build the project:

   ```sh
   cargo build
   ```

## Running tests

```sh
cargo test
```

Tests live in two places:

- Unit tests in `src/lib.rs` — cover core encode/decode logic
- Integration tests in `tests/cli.rs` — exercise the binary end-to-end

## Code style

Before opening a pull request, run:

```sh
cargo fmt
cargo clippy -- -D warnings
```

Fix any warnings that `clippy` reports. CI enforces both.

## Branching and pull requests

- Create a feature branch off `main`:

  ```sh
  git checkout -b my-feature
  ```

- Open a pull request against `main` on GitHub.
- Keep pull requests focused — one logical change per PR.

## Commit messages

Use a short imperative subject line (under 72 characters), for example:

```
Add decode error handling
Fix stdin detection on Windows
Bump percent-encoding to 2.3
```

Avoid past tense ("Added", "Fixed") and vague messages ("Fix stuff").

## Reporting bugs

Open an issue at https://github.com/arcaartem/urlendec/issues. Include:

- The `urlendec` version (`urlendec --version`)
- The command you ran and the input you provided
- The actual output and the output you expected
- Your operating system and architecture

## Licensing

By submitting a contribution you agree that your work will be licensed under the
[MIT License](LICENSE), the same license as this project.
