use anyhow::Result;
use clap::Parser;
use midly::Smf;
use std::fs::{self, File};
use std::path::PathBuf;

mod process;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    midi: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let bytes = fs::read(args.midi).unwrap();
    let smf = Smf::parse(&bytes).unwrap();

    let output = process::process(smf)?;

    let file = File::create(args.output)?;
    serde_json::to_writer_pretty(file, &output)?;

    Ok(())
}
