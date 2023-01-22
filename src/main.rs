use anyhow::Result;
use clap::Parser;
use log::debug;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use urlencoding;

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

fn main() -> io::Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    debug!(
        "input arguments: decode='{}'  input_string='{}'  input_file='{}'  output_file='{}'",
        cli.decode, cli.input_string, cli.input_file, cli.output_file
    );

    if cli.input_string.is_empty() {
        debug!("processing multi-line input");
        encode_decode_multiline(&cli);
    } else {
        debug!("processing single-line input");
        encode_decode_singleline(&cli);
    }

    Ok(())
}

fn encode_decode_singleline(cli: &Cli) {
    debug!("encode/decode single-line input");
    let mut writer = create_writer(&cli.output_file);

    let output = encode_decode(&cli, &cli.input_string);

    writer.write_fmt(format_args!("{}", output)).unwrap();
}

fn encode_decode_multiline(cli: &Cli) {
    debug!("encode/decode multi-line input");
    let reader = create_reader(&cli.input_file).expect("Cannot create reader");

    let mut writer = create_writer(&cli.output_file);

    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        if last_input.len() == 0 {
            break;
        }

        let output = encode_decode(&cli, &last_input);

        writer.write_fmt(format_args!("{}\n", output)).unwrap();
    }
}

fn encode_decode(cli: &Cli, input: &String) -> String {
    debug!("encode/decode input");
    let result: String = match cli.decode {
        true => urlencoding::decode(&input).expect("UTF-8").to_string(),
        false => urlencoding::encode(&input).to_string(),
    };
    result
}

fn create_writer(output_file: &String) -> Box<dyn Write> {
    debug!("creating writer for output file '{}'", output_file);
    let writer: Box<dyn Write>;

    if output_file == "-" {
        writer = Box::new(BufWriter::new(io::stdout().lock()));
    } else {
        writer = Box::new(BufWriter::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(output_file)
                .unwrap(),
        ));
    }
    writer
}

fn create_reader(input_file: &String) -> Result<Box<dyn BufRead>> {
    debug!("creating reader for input file '{}'", input_file);
    let reader: Box<dyn BufRead>;

    if input_file == "-" {
        reader = Box::new(BufReader::new(io::stdin().lock()));
    } else {
        let file =
            File::open(input_file).expect(format!("could not read file `{}`", input_file).as_str());
        reader = Box::new(BufReader::new(file));
    };
    Ok(reader)
}
