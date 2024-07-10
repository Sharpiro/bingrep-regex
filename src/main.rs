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

use anyhow::Result;
use clap::Parser;
use filter::filter;
use num_format::{Locale, ToFormattedString};
use options::get_context_range;
use parser::parse_binary_syntax;
use pretty_hex::{HexConfig, PrettyHex};
use std::io::{stderr, Read};
use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;

mod filter;
mod options;
mod parser;

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
    /// Experimental binary syntax with partial regex support. E.g. "dead be [ef|ed]"
    #[arg(short, long)]
    binary_syntax: bool,
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

    let resolved_pattern = if args.binary_syntax {
        parse_binary_syntax(&args.pattern)
    } else {
        args.pattern
    };
    debug!(
        "resolved pattern: {}, len: {}",
        &resolved_pattern,
        resolved_pattern.len()
    );

    let max_matches = args.limit.unwrap_or(usize::MAX);
    let matches = filter(&resolved_pattern, &buffer)?;
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
