/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-04-30
 */
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use tokio::io::Result;

use options::Options;

mod options;

#[tokio::main]
async fn main() -> ExitCode {
    let options = Options::parse();
    return if let Some(folder) = options.folder() {
        if options.summary() {
            match show_summary(&folder).await {
                Err(error) => {
                    eprintln!("Error: {}", error);
                    ExitCode::from(1)
                }
                _ => ExitCode::default(),
            }
        } else {
            match show_fortune(&folder, options.verbose()).await {
                Err(error) => {
                    eprintln!("Error: {}", error);
                    ExitCode::from(1)
                }
                _ => ExitCode::default(),
            }
        }
    } else {
        eprintln!("Can't find default data folder");
        ExitCode::from(100)
    };
}
async fn show_summary(folder: &PathBuf) -> Result<()> {
    let fortune_stats = shared_library::fortune_stats(folder).await?;
    let total = fortune_stats.iter().fold(0, |total, fortune_file_info| {
        total + fortune_file_info.count()
    });
    println!(
        "{} fortune{} in {} file{}",
        total,
        if total == 1 { "" } else { "s" },
        fortune_stats.len(),
        if fortune_stats.len() == 1 { "" } else { "s" },
    );
    println!("{}", folder.display());
    let longest = &fortune_stats.iter().fold(0, |acc, fortune_file_info| {
        let file = fortune_file_info.file();
        std::cmp::max(acc, file.len())
    });
    for fortune_status in &fortune_stats {
        let file_name = fortune_status.file();
        let percent = ((fortune_status.count() * 100) as f64) / (total as f64);
        println!("\t{:width$} {:.2}%", file_name, percent, width = longest);
    }
    Ok(())
}
async fn show_fortune(data_path: &PathBuf, verbose: bool) -> Result<()> {
    let fortune = shared_library::random_fortune(data_path).await?;
    if verbose {
        println!("[{}]", fortune.file())
    }
    println!("{}", fortune.fortune());
    Ok(())
}
