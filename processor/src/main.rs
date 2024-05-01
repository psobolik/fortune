/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-04-29
 */

use std::path::Path;
use std::process::ExitCode;

use clap::Parser;
use tokio::fs;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

use options::Options;
use shared_library::{DataFile, Flags, Header};

mod options;

/// This program read a `fortune cookie` file and builds an index file for it.
/// The index file should be compatible with the Unix `fortune` program.
#[tokio::main]
async fn main() -> ExitCode {
    // This app will set the Random, Ordered or Rotated flags in the output file,
    // but otherwise the file will be the same. If you want the data in some particular
    // order or rotated, do it in the input file.
    // TIP: Use this command to ROT13 the file:
    // $ cat scratch | tr '[a-z][A-Z]' '[n-za-m][N-ZA-M]' >scratch.rot

    let options = Options::parse();

    match process(
        options.file(),
        options.dat_file(),
        Some(options.separator()),
        Some(options.flags()),
        options.quiet(),
    )
    .await
    {
        Err(error) => {
            eprintln!("Error: {}", error);
            ExitCode::from(1)
        }
        _ => ExitCode::default(),
    }
}

async fn process<P: AsRef<Path>>(
    in_file: P,
    out_file: P,
    separator: Option<char>,
    flags: Option<Flags>,
    quiet: bool,
) -> io::Result<()> {
    let data_file = build_data_file(&in_file, separator, flags).await?;
    write_data_file(&data_file, &out_file).await?;
    if !quiet {
        show_summary(&data_file, &in_file, &out_file)
    };
    Ok(())
}
fn show_summary<P: AsRef<Path>>(data_file: &DataFile, in_file: P, out_file: P) {
    println!("Processed file:  {}", in_file.as_ref().display());
    println!("Generated file:  {}", out_file.as_ref().display());
    println!("Number of items: {}", data_file.header.count());
    println!("Flags:           [{}]", data_file.header.flags());
    println!("Shortest:        {}", data_file.header.shortest());
    println!("Longest:         {}", data_file.header.longest());
}
async fn write_data_file<P: AsRef<Path>>(data_file: &DataFile, out_file: P) -> io::Result<()> {
    let mut file = fs::File::create(out_file).await?;
    file.write_all(data_file.to_bytes().as_ref()).await?;
    Ok(())
}

async fn build_data_file<P: AsRef<Path>>(
    in_file: P,
    separator: Option<char>,
    flags: Option<Flags>,
) -> io::Result<DataFile> {
    let separator = separator.unwrap_or('%');
    let flags = flags.unwrap_or_default();
    let end_line = format!("{separator}\n");

    let mut count = u32::MIN;
    let mut shortest = u32::MAX;
    let mut longest = u32::MIN;
    let mut offsets: Vec<u32> = vec![];

    let in_file = fs::File::open(in_file).await?;
    let mut reader = io::BufReader::new(in_file);

    let mut line = String::new();
    let mut len = 0;
    let mut offset = 0;

    while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
        let line_len = line.len() as u32;
        if line != end_line {
            len += line_len;
        } else {
            // End of the entry
            if len < shortest {
                shortest = len;
            } else if len > longest {
                longest = len;
            }
            offsets.push(offset);
            offset += len + line_len;
            count += 1;
            len = 0;
        }
        line.clear();
    }
    offsets.push(offset); // End of file offset
    let header = Header::default()
        .set_separator(separator)
        .set_count(count)
        .set_longest(longest)
        .set_shortest(shortest)
        .set_flags(flags)
        .to_owned();
    Ok(DataFile { header, offsets })
}
