/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-05-01
 */
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    /// The fortune folder
    folder: Option<PathBuf>,

    /// Show information about the fortune files and exit
    #[arg(short, long)]
    summary: bool,

    /// Also show the fortune's source file
    #[arg(short, long)]
    verbose: bool,
}

impl Options {
    pub fn folder(&self) -> Option<PathBuf> {
        if let Some(file) = &self.folder {
            Some(PathBuf::from(file))
        } else {
            directories::ProjectDirs::from("home", "psobolik", "fortune")
                .map(|project_dirs| PathBuf::from(project_dirs.data_dir()))
        }
    }
    pub fn summary(&self) -> bool {
        self.summary
    }
    pub fn verbose(&self) -> bool {
        self.verbose
    }
}
