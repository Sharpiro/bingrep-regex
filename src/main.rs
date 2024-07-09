#![warn(clippy::pedantic)]
#![warn(clippy::never_loop)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::panic)]
#![warn(clippy::todo)]
#![warn(clippy::expect_used)]
#![allow(clippy::let_and_return)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_precision_loss)]

use anyhow::{Context, Result};
use clap::Parser;
use filter::filter;
use iter_tools::Itertools;
use num_format::{Locale, ToFormattedString};
use options::get_context_range;
use pretty_hex::{HexConfig, PrettyHex};
use std::{
    borrow::Cow,
    io::{stderr, Read},
};
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

mod filter;
mod options;

/// Search binary files with regex
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Pattern
    pattern: String,
    /// File
    file: Option<String>,
    /// Show additional lines of context
    #[arg(short,long, value_parser = parse_hex_or_digit)]
    context: Option<usize>,
    /// Treat pattern as raw binary. E.g. "de ad be ef"
    #[arg(short, long)]
    raw: bool,
    /// Limit number of matches to display
    #[arg(short, long)]
    limit: Option<usize>,
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
        get_raw_pattern(&args.pattern)
    } else {
        args.pattern
    };
    debug!("pattern: {}", &pattern);

    let max_matches = args.limit.unwrap_or(usize::MAX);
    let matches = filter(&pattern, &buffer)?;
    for (i, &(start, end)) in matches.iter().take(max_matches).enumerate() {
        let cfg = HexConfig {
            title: true,
            ascii: false,
            width: 16,
            group: 4,
            chunk: 1,
            ..Default::default()
        };

        let match_len = end - start;
        let match_details = if let Some(context) = args.context {
            let (display_start, display_end) = get_context_range(start, end, context);
            let slice = buffer
                .get(display_start..display_end)
                .ok_or(anyhow::anyhow!("out of bounds"))?;
            let cfg = HexConfig {
                display_offset: display_start,
                ..cfg
            };
            Some((slice, cfg))
        } else {
            None
        };

        println!("{n}: [{start:#x}, {end:#x}): {match_len}", n = i + 1);
        if let Some((slice, cfg)) = match_details {
            println!("{:?}", slice.hex_conf(cfg));
        }
    }

    Ok(())
}

fn parse_hex_or_digit(arg: &str) -> Result<usize> {
    if let Some((_, arg)) = arg.split_once("0x") {
        return Ok(usize::from_str_radix(arg, 16)?);
    }

    Ok(arg.parse()?)
}

fn get_raw_pattern(pattern: &str) -> String {
    pattern
        .trim()
        .split(' ')
        .map(|v| {
            let is_hex = u8::from_str_radix(v, 16).is_ok();
            if is_hex {
                Cow::from(format!(r"\x{v}"))
            } else {
                Cow::from(v)
            }
        })
        .join("")
}
