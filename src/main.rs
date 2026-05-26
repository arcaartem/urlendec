use anyhow::Result;
use clap::Parser;
use log::debug;
use std::io::BufRead;
use urlendec::{create_reader, create_writer, encode_decode};

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

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    debug!(
        "input arguments: decode='{}'  input_string='{}'  input_file='{}'  output_file='{}'",
        cli.decode, cli.input_string, cli.input_file, cli.output_file
    );

    let reader = create_reader(&cli.input_file, &cli.input_string)?;
    let mut writer = create_writer(&cli.output_file)?;

    for line in reader.lines() {
        let output = encode_decode(cli.decode, &line?)?;
        writeln!(writer, "{}", output)?;
    }
    writer.flush()?;

    Ok(())
}
