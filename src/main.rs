#![warn(clippy::pedantic)]
#![warn(clippy::never_loop)]

use anyhow::Result;
use bingrep::filter::filter;
use clap::Parser;
use pretty_hex::{HexConfig, PrettyHex};
use std::io::Read;

/// Search binaries
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Pattern
    pattern: String,
    /// File
    file: Option<String>,
    /// Number of times to greet
    #[arg(long, default_value_t = 1)]
    count: usize,
    /// Show context of matches
    #[arg(short, long)]
    context: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut buffer = Vec::<u8>::new();
    let read_size = if let Some(file_path) = args.file {
        let mut file = std::fs::File::open(file_path)?;
        file.read_to_end(&mut buffer)?
    } else {
        std::io::stdin().read_to_end(&mut buffer)?
    };

    dbg!(read_size);

    let matches = filter(&args.pattern, &buffer)?;
    for (i, &(start, end)) in matches.iter().enumerate() {
        if args.context {
            let cfg = HexConfig {
                title: true,
                ascii: false,
                width: 16,
                group: 4,
                chunk: 1,
                display_offset: start,
                max_bytes: 256,
            };
            let slice = &buffer[start..end];
            // let slice = &buffer[start..start + 256];
            println!(
                "Match {i}/{len}: ({start:#x}, {end:#x}), {dump:?}",
                i = i + 1,
                len = matches.len(),
                dump = slice.hex_conf(cfg)
            );
        } else {
            println!(
                "Match {i}/{len}: ({start:#x}, {end:#x})",
                i = i + 1,
                len = matches.len()
            );
        }
    }

    Ok(())
}
