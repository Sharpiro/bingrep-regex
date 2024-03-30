#![warn(clippy::pedantic)]
#![warn(clippy::never_loop)]

use anyhow::{anyhow, Result};
use bingrep::filter::filter;
use iter_tools::Itertools;
use pretty_hex::{HexConfig, PrettyHex};
use std::io::Read;

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect_vec();
    let pattern = args.first().ok_or(anyhow!("missing pattern"))?;

    let mut buffer = Vec::<u8>::new();
    let read_size = if let Some(file_path) = args.get(1) {
        println!("file input: {pattern} {file_path}");
        let mut file = std::fs::File::open(file_path)?;
        file.read_to_end(&mut buffer)?
    } else {
        println!("stdin input: {pattern}");
        std::io::stdin().read_to_end(&mut buffer)?
    };

    println!("read: {read_size}");
    // println!("{buffer:#04x?}");

    let matches = filter(pattern, &buffer)?;
    for (i, &(start, end)) in matches.iter().enumerate() {
        // println!("{rmatch:x?}: {slice:?}");
        // let &(start, end) = rmatch;
        let slice = &buffer[start..end];
        println!(
            "Match {i}/{len}: ({start:#x}, {end:#x})",
            i = i + 1,
            len = matches.len()
        );
        let cfg = HexConfig {
            title: true,
            ascii: false,
            width: 16,
            group: 4,
            chunk: 1,
            display_offset: start,
            ..HexConfig::default()
        };
        println!("{:?}", slice.hex_conf(cfg));
    }
    dbg!(&matches.len());

    Ok(())
}
