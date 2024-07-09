#![warn(clippy::pedantic)]
#![warn(clippy::never_loop)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::panic)]
#![warn(clippy::todo)]
#![warn(clippy::expect_used)]
#![allow(clippy::let_and_return)]
#![allow(clippy::missing_errors_doc)]

use anyhow::{Context, Result};
use clap::Parser;
use filter::filter;
use iter_tools::Itertools;
use num_format::{Locale, ToFormattedString};
use pretty_hex::{HexConfig, PrettyHex};
use std::{
    cmp::max,
    io::{stderr, Read},
};
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

mod filter;

/// Search binary files with regex
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
    /// Treat pattern as raw binary. E.g. "de ad be ef"
    #[arg(short, long)]
    raw: bool,
    /// Max number of matches to display
    #[arg(short = 'M', long, default_value_t = 100)]
    max_matches: usize,
}

fn main() -> Result<()> {
    let log_level = if cfg!(debug_assertions) {
        Level::TRACE
    } else {
        Level::ERROR
    };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_writer(stderr)
        .finish();

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

    let pattern = if args.raw {
        get_raw_pattern(&args.pattern)?
    } else {
        args.pattern
    };
    debug!("pattern: {}", &pattern);
    let matches = filter(&pattern, &buffer)?;
    for (i, &(start, end)) in matches.iter().take(args.max_matches).enumerate() {
        let cfg = HexConfig {
            title: true,
            ascii: false,
            width: 16,
            group: 4,
            chunk: 1,
            max_bytes: 256,
            ..Default::default()
        };

        let match_details = if let Some(context) = args.context {
            let display_start = start.checked_sub(context).unwrap_or_default();
            let display_end = context
                .checked_add(0x10)
                .and_then(|v| start.checked_add(v))
                .unwrap_or_default();
            let display_end = max(display_end, context);
            let slice = buffer
                .get(display_start..display_end)
                .ok_or(anyhow::anyhow!("out of bounds"))?;
            let cfg = HexConfig {
                display_offset: display_start,
                ..cfg
            };
            Some((slice, cfg))
        } else if args.r#match {
            let slice = buffer
                .get(start..end)
                .ok_or(anyhow::anyhow!("out of bounds"))?;
            let cfg = HexConfig {
                display_offset: start,
                ..cfg
            };
            Some((slice, cfg))
        } else {
            None
        };

        println!(
            "{n}: [{start:#x}, {end:#x}): {len}",
            n = i + 1,
            len = end - start
        );
        if let Some((slice, cfg)) = match_details {
            println!("{:?}", slice.hex_conf(cfg));
        }
    }

    if matches.len() > args.max_matches {
        println!("...");
    }

    Ok(())
}

fn parse_hex_or_digit(arg: &str) -> Result<usize> {
    if let Some((_, arg)) = arg.split_once("0x") {
        return Ok(usize::from_str_radix(arg, 16)?);
    }

    Ok(arg.parse()?)
}

fn get_raw_pattern(pattern: &str) -> Result<String> {
    let bytes: Vec<u8> = pattern
        .trim()
        .split(' ')
        .map(|v| u8::from_str_radix(v, 16))
        .try_collect()
        .context("invalid raw bytes pattern")?;
    let formatted_bytes = bytes.iter().map(|&v| format!(r"\x{v:02x}")).join("");

    Ok(formatted_bytes)
}
