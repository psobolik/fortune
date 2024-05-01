/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-04-30
 */
use std::path::PathBuf;

use clap::Parser;

use shared_library::Flags;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    /// The fortune file to process
    file: PathBuf,

    /// The name for the generated .dat file [default: <FILE>.dat]
    dat_file: Option<PathBuf>,

    /// Quiet, i.e. don't show summary
    #[arg(short, long)]
    quiet: bool,

    /// The separator character used in the fortune file
    #[arg(short, long, default_value = "%")]
    separator: char,

    /// Set the Random flag
    #[arg(long)]
    random: bool,

    /// Set the Ordered flag
    #[arg(long)]
    ordered: bool,

    /// Set the Rotated flag
    #[arg(long)]
    rotated: bool,
}

#[allow(dead_code)]
impl Options {
    pub fn file(&self) -> PathBuf {
        PathBuf::from(&self.file)
    }
    pub fn dat_file(&self) -> PathBuf {
        if let Some(dat_file) = &self.dat_file {
            PathBuf::from(dat_file)
        } else {
            PathBuf::from(&self.file)
                .with_extension("")
                .with_extension("dat")
        }
    }
    pub fn quiet(&self) -> bool {
        self.quiet
    }
    pub fn separator(&self) -> char {
        self.separator
    }
    pub fn flags(&self) -> Flags {
        let mut flags = Flags::empty();
        if self.random {
            flags |= Flags::Random;
        }
        if self.ordered {
            flags |= Flags::Ordered;
        }
        if self.rotated {
            flags |= Flags::Rotated;
        }
        flags
    }
}
