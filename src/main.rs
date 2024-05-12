use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use image::io::Reader;

#[derive(Parser)]
struct Cli {
    image: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let image = Reader::open(cli.image)?.decode()?;
    println!("{} {}", image.width(), image.height());

    Ok(())
}
