use anyhow::{Context, Result};
use clap::Parser;
use log::debug;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use urlencoding;

enum InputType {
    File,
    String,
    Stdio,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "false")]
    decode: bool,

    #[arg(short = 's', long = "string", default_value = "", group = "input")]
    input_string: String,

    #[arg(short, long, default_value = "-", group = "input")]
    input_file: String,

    #[arg(short, long, default_value = "-")]
    output_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = Cli::parse();

    debug!(
        "input arguments: decode='{}'  input_string='{}'  input_file='{}'  output_file='{}'",
        cli.decode, cli.input_string, cli.input_file, cli.output_file
    );

    let reader = create_reader(&cli)?;
    let mut writer = create_writer(&cli)?;

    for line in reader.lines() {
        let output = encode_decode(&cli, &line?)?;

        writer.write_fmt(format_args!("{}\n", output))?;
    }
    writer.flush()?;

    Ok(())
}

fn encode_decode(cli: &Cli, input: &String) -> Result<String> {
    Ok(if cli.decode {
        debug!("decode input: '{}'", input);
        urlencoding::decode(&input)?.to_string()
    } else {
        debug!("encode input: '{}'", input);
        urlencoding::encode(&input).to_string()
    })
}

fn create_writer(cli: &Cli) -> Result<Box<dyn Write>> {
    debug!("creating writer for output file '{}'", cli.output_file);
    let writer: Box<dyn Write>;

    if cli.output_file == "-" {
        writer = Box::new(BufWriter::new(io::stdout().lock()));
    } else {
        writer = Box::new(BufWriter::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&cli.output_file)
                .with_context(|| format!("could not create output file `{}`", cli.output_file))?,
        ));
    }
    Ok(writer)
}

fn create_reader(cli: &Cli) -> Result<Box<dyn BufRead + '_>> {
    Ok(match get_input_type(&cli) {
        InputType::File => Box::new(BufReader::new(
            File::open(&cli.input_file)
                .with_context(|| format!("could not read file `{}`", cli.input_file))?,
        )),
        InputType::Stdio => Box::new(BufReader::new(io::stdin().lock())),
        InputType::String => Box::new(BufReader::new(cli.input_string.as_bytes())),
    })
}

fn get_input_type(cli: &Cli) -> InputType {
    if cli.input_string.is_empty() {
        if cli.input_file == "-" {
            InputType::Stdio
        } else {
            InputType::File
        }
    } else {
        InputType::String
    }
}
