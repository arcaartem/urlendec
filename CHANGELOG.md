# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-05-26

### Added
- LICENSE, CONTRIBUTING, expanded README, and CHANGELOG
- Strict validation of percent-encoded input on decode (rejects invalid
  escapes like `%ZZ`, `%4`, or a trailing `%`)
- CLI help text on every flag
- Integration coverage for Unicode input, empty files, decode-to-file,
  mid-stream decode failure, and clap's mutually-exclusive `-s`/`-i`

### Changed
- Refactored library into `lib.rs` + thin `main.rs`
- Bumped dependencies to current versions
- Modernized GitHub Actions workflows (matrix strategy, actions v4)
- Updated repository URLs from cobalt-forge to arcaartem
- Consolidated unit tests into table-driven form; made `InputType` and
  `get_input_type` private (internal to `create_reader`)

## [0.1.1] - 2023-06

### Added
- Man page (`doc/urlendec.1.md`) distributed with binary releases

### Changed
- Refactored release workflow

## [0.1.0] - 2023-02

### Added
- Initial release: URL encode/decode via `-s`, `-i`, `-o`, and `-d` flags
- Line-by-line processing of files and stdin

[Unreleased]: https://github.com/arcaartem/urlendec/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/arcaartem/urlendec/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/arcaartem/urlendec/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/arcaartem/urlendec/releases/tag/v0.1.0
