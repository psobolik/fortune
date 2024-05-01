/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-05-02
 */
use std::io;
use std::io::{Error, ErrorKind, SeekFrom};
use std::path::PathBuf;

use rand::Rng;
use tokio::fs;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, Result};

pub use data_file::{DataFile, Flags, Header};
pub use fortune::Fortune;
pub use fortune_file_info::FortuneFileInfo;
pub use fortune_stats::FortuneStats;

pub mod data_file;
pub mod fortune;
pub mod fortune_file_info;
pub mod fortune_stats;

/// Returns a fortune chosen randomly from all the fortune files in the given path.
pub async fn random_fortune(data_path: &PathBuf) -> Result<Fortune> {
    let fortune_files = fortune_files(data_path).await?;

    // Select a random index between 0 and the total number of fortunes
    let total = fortune_files.iter().fold(0, |total, fortune_file_info| {
        total + fortune_file_info.count()
    });
    let mut index = rand::thread_rng().gen_range(0..total);

    // Find the fortune file that contains the selected index, and calculate the index of the
    // fortune's offset in that file
    let fortune_file_info = fortune_files.iter().find(|fortune_file_info| {
        let count = fortune_file_info.count();
        if count < index {
            index -= count;
            false
        } else {
            true
        }
    });
    // Return the text between the offsets in the fortune file.
    if let Some(fortune_file_info) = fortune_file_info {
        let start = fortune_file_info.offset_at(index) as u64;
        let end = fortune_file_info.offset_at(index + 1) as u64;
        let fortune = get_fortune(&fortune_file_info.path.with_extension(""), start, end).await?;
        Ok(Fortune::new(fortune_file_info.fortune_file(), fortune))
    } else {
        Err(Error::new(ErrorKind::Other, "This should never happen"))
    }
}
/// Returns an array of [FortuneStats], with an entry for each of the fortune files in the given path.
pub async fn fortune_stats(data_path: &PathBuf) -> Result<Vec<FortuneStats>> {
    match fortune_files(data_path).await {
        Ok(fortune_file_infos) => {
            let fortune_stats = fortune_file_infos
                .iter()
                .map(|fortune_file_info| {
                    FortuneStats::new(fortune_file_info.fortune_file(), fortune_file_info.count())
                })
                .collect();
            Ok(fortune_stats)
        }
        Err(error) => {
            // Enhance error message
            let message = format!(
                "Cannot read data folder '{}': {}",
                data_path.display(),
                error
            );
            let error = io::Error::new(error.kind(), message);
            Err(error)
        }
    }
}
async fn fortune_files(data_path: &PathBuf) -> Result<Vec<FortuneFileInfo>> {
    match fs::read_dir(data_path).await {
        Ok(mut entries) => {
            let mut data_files: Vec<FortuneFileInfo> = vec![];
            while let Some(entry) = entries.next_entry().await? {
                let entry = entry.path();
                if !entry.is_file() {
                    continue;
                }
                if let Some(extension) = entry.extension() {
                    if extension != "dat" {
                        continue;
                    }
                    if let Some(data_file) = read_data_file(&entry).await? {
                        data_files.push(FortuneFileInfo::new(entry, data_file));
                    }
                }
            }
            Ok(data_files)
        }
        Err(error) => {
            // Enhance error message
            let message = format!(
                "Cannot read data folder '{}': {}",
                data_path.display(),
                error
            );
            let error = io::Error::new(error.kind(), message);
            Err(error)
        }
    }
}
async fn get_fortune(path: &PathBuf, start: u64, end: u64) -> Result<String> {
    let len = (end - start - 2) as usize;
    let mut bucket = vec![0u8; len];

    let mut file = File::open(path).await?;
    file.seek(SeekFrom::Start(start)).await?;
    file.read_exact(&mut bucket).await?;
    Ok(String::from_utf8(bucket).unwrap_or_default())
}
async fn read_data_file(in_file: &PathBuf) -> Result<Option<DataFile>> {
    let mut data_file = DataFile::default();

    let mut file = File::open(in_file).await?;

    let version = file.read_u32().await?;
    if version != data_file.header.version() {
        return Ok(None);
    }

    let count = file.read_u32().await?;
    if count == 0 {
        return Ok(None);
    }

    let longest = file.read_u32().await?;
    let shortest = file.read_u32().await?;
    let bits = file.read_u32().await?;
    let flags = Flags::from_bits(bits).unwrap_or_default();
    let bits = file.read_u32().await?;
    let separator = bits.to_be_bytes()[0] as char;

    data_file.header = *data_file
        .header
        .set_count(count)
        .set_longest(longest)
        .set_shortest(shortest)
        .set_flags(flags)
        .set_separator(separator);
    for _ in 0..=count {
        let offset = file.read_u32().await?;
        data_file.offsets.push(offset);
    }
    Ok(Some(data_file))
}
