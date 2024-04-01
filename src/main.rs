#![warn(clippy::pedantic)]
#![warn(clippy::never_loop)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::panic)]
#![warn(clippy::todo)]
#![warn(clippy::expect_used)]
#![allow(clippy::let_and_return)]
#![allow(clippy::missing_errors_doc)]

use anyhow::Result;
use clap::Parser;
use filter::filter;
use num_format::{Locale, ToFormattedString};
use pretty_hex::{HexConfig, PrettyHex};
use std::io::Read;
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

mod filter;

const MAX_MATCH_COUNT: usize = 100;

fn main() -> Result<()> {
    let log_level = if cfg!(debug_assertions) {
        Level::TRACE
    } else {
        Level::ERROR
    };
    let subscriber = FmtSubscriber::builder().with_max_level(log_level).finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let args = Args::parse();
    let mut buffer = Vec::<u8>::new();
    let read_size = if let Some(file_path) = args.file {
        let mut file = std::fs::File::open(file_path)?;
        file.read_to_end(&mut buffer)?
    } else {
        std::io::stdin().read_to_end(&mut buffer)?
    };

    debug!("file_size: {}", read_size.to_formatted_string(&Locale::en));

    // @todo: doesn't stream matches
    let matches = filter(&args.pattern, &buffer)?;
    for (i, &(start, end)) in matches.iter().take(MAX_MATCH_COUNT).enumerate() {
        let cfg = HexConfig {
            title: true,
            ascii: false,
            width: 16,
            group: 4,
            chunk: 1,
            max_bytes: 256,
            ..Default::default()
        };

        // @todo: doesn't work if context input is less than the start of the file
        if let Some(context) = args.context {
            let display_offset = start - 0x10;
            let slice = &buffer
                .get(display_offset..start + (context - 0x10))
                .ok_or(anyhow::anyhow!("out of bounds"))?;
            let cfg = HexConfig {
                display_offset,
                ..cfg
            };
            println!(
                "Match {i}/{len}: ({start:#x}, {end:#x}), {dump:?}",
                i = i + 1,
                len = matches.len(),
                dump = slice.hex_conf(cfg)
            );
        } else if args.r#match {
            let slice = &buffer
                .get(start..end)
                .ok_or(anyhow::anyhow!("out of bounds"))?;
            let cfg = HexConfig {
                display_offset: start,
                ..cfg
            };
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

    if matches.len() > MAX_MATCH_COUNT {
        let hidden = matches.len() - MAX_MATCH_COUNT;
        println!("{hidden} matches hidden...");
    }

    Ok(())
}

/// Search binaries
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Pattern
    pattern: String,
    /// File
    file: Option<String>,
    /// Show matches in binary
    #[arg(short, long)]
    r#match: bool,
    /// Show matches with provided context size
    #[arg(short,long, value_parser = parse_hex_or_digit)]
    context: Option<usize>,
}

fn parse_hex_or_digit(arg: &str) -> Result<usize> {
    if let Some((_, arg)) = arg.split_once("0x") {
        return Ok(usize::from_str_radix(arg, 16)?);
    }

    Ok(arg.parse()?)
}
