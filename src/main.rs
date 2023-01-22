use clap::Parser;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use urlencoding;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "false")]
    decode: bool,

    #[arg(short, long, default_value = "-")]
    input_file: String,

    #[arg(short, long, default_value = "-")]
    output_file: String,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let reader = create_reader(&cli.input_file);

    let mut writer = create_writer(&cli.output_file);

    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        if last_input.len() == 0 {
            break;
        }

        let output = encode_decode(&cli.decode, last_input);

        writer.write_fmt(format_args!("{}\n", output)).unwrap();
    }

    Ok(())
}

fn encode_decode(decode: &bool, last_input: String) -> String {
    let result: String = match decode {
        true => urlencoding::decode(&last_input).expect("UTF-8").to_string(),
        false => urlencoding::encode(&last_input).to_string(),
    };
    result
}

fn create_writer(output_file: &String) -> Box<dyn Write> {
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

fn create_reader(input_file: &String) -> Box<dyn BufRead> {
    let reader: Box<dyn BufRead>;

    if input_file == "-" {
        reader = Box::new(BufReader::new(io::stdin().lock()));
    } else {
        reader = Box::new(BufReader::new(fs::File::open(input_file).unwrap()));
    };
    reader
}
