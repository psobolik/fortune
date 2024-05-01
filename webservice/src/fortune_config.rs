/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-05-03
 */
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct FortuneConfig {
    pub data_path: PathBuf,
}
impl FortuneConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name(CONFIG_FILE))
            .build()?;
        builder.try_deserialize()
    }
}
const CONFIG_FILE: &str = "./Config.toml";
