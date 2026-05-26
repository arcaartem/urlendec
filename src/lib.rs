use anyhow::{Context, Result};
use log::debug;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};

enum InputType {
    File,
    String,
    Stdio,
}

fn validate_percent_encoding(input: &str) -> Result<()> {
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if i + 2 >= bytes.len() {
                anyhow::bail!(
                    "invalid percent-encoding: incomplete escape sequence at end of input"
                );
            }
            let h1 = bytes[i + 1];
            let h2 = bytes[i + 2];
            if !h1.is_ascii_hexdigit() || !h2.is_ascii_hexdigit() {
                anyhow::bail!(
                    "invalid percent-encoding: '%{}{}' is not a valid escape sequence",
                    h1 as char,
                    h2 as char
                );
            }
            i += 3;
        } else {
            i += 1;
        }
    }
    Ok(())
}

pub fn encode_decode(decode: bool, input: &str) -> Result<String> {
    Ok(if decode {
        debug!("decode input: '{}'", input);
        validate_percent_encoding(input)?;
        urlencoding::decode(input)?.to_string()
    } else {
        debug!("encode input: '{}'", input);
        urlencoding::encode(input).to_string()
    })
}

pub fn create_writer(output_file: &str) -> Result<Box<dyn Write>> {
    debug!("creating writer for output file '{}'", output_file);
    if output_file == "-" {
        Ok(Box::new(BufWriter::new(io::stdout().lock())))
    } else {
        Ok(Box::new(BufWriter::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(output_file)
                .with_context(|| format!("could not create output file `{}`", output_file))?,
        )))
    }
}

pub fn create_reader<'a>(input_file: &str, input_string: &'a str) -> Result<Box<dyn BufRead + 'a>> {
    Ok(match get_input_type(input_file, input_string) {
        InputType::File => Box::new(BufReader::new(
            File::open(input_file)
                .with_context(|| format!("could not read file `{}`", input_file))?,
        )),
        InputType::Stdio => Box::new(BufReader::new(io::stdin().lock())),
        InputType::String => Box::new(BufReader::new(input_string.as_bytes())),
    })
}

fn get_input_type(input_file: &str, input_string: &str) -> InputType {
    if input_string.is_empty() {
        if input_file == "-" {
            InputType::Stdio
        } else {
            InputType::File
        }
    } else {
        InputType::String
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_produces_expected_output() {
        let cases: &[(&str, &str)] = &[
            ("Hello, world!", "Hello%2C%20world%21"),
            ("café", "caf%C3%A9"),
            ("", ""),
        ];
        for (input, expected) in cases {
            assert_eq!(
                encode_decode(false, input).unwrap(),
                *expected,
                "input: {input:?}"
            );
        }
    }

    #[test]
    fn encode_decode_round_trip() {
        let cases: &[&str] = &["Hello, world!", "café 你好"];
        for original in cases {
            let encoded = encode_decode(false, original).unwrap();
            let decoded = encode_decode(true, &encoded).unwrap();
            assert_eq!(decoded, *original, "round-trip failed for: {original:?}");
        }
    }

    #[test]
    fn encode_decode_invalid_utf8_returns_err() {
        // %80 decodes to 0x80, which is not valid UTF-8 on its own
        let result = encode_decode(true, "%80");
        assert!(result.is_err());
    }

    #[test]
    fn decode_rejects_invalid_percent_encoding() {
        let cases: &[(&str, &str)] = &[
            ("%ZZ", "non-hex digits"),
            ("abc%", "lone percent at end"),
            ("%4", "partial escape (1 hex digit)"),
            ("%4Z", "mixed valid/invalid hex digits"),
        ];
        for (input, description) in cases {
            let result = encode_decode(true, input);
            assert!(result.is_err(), "expected error for: {description}");
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("invalid percent-encoding"),
                "expected 'invalid percent-encoding' in error for: {description}"
            );
        }
    }

    #[test]
    fn decode_accepts_valid_inputs() {
        let cases: &[(&str, &str)] =
            &[("%41", "A"), ("%2C", ","), ("%2c", ","), ("hello", "hello")];
        for (input, expected) in cases {
            assert_eq!(
                encode_decode(true, input).unwrap(),
                *expected,
                "input: {input:?}"
            );
        }
    }

    #[test]
    fn get_input_type_stdio() {
        assert!(matches!(get_input_type("-", ""), InputType::Stdio));
    }

    #[test]
    fn get_input_type_file() {
        assert!(matches!(get_input_type("file.txt", ""), InputType::File));
    }

    #[test]
    fn get_input_type_string_takes_precedence_over_stdin() {
        assert!(matches!(get_input_type("-", "hello"), InputType::String));
    }

    #[test]
    fn get_input_type_string_takes_precedence_over_file() {
        assert!(matches!(
            get_input_type("file.txt", "hello"),
            InputType::String
        ));
    }
}
