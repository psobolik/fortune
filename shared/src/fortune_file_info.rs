/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-05-02
 */
use std::path::PathBuf;

use super::DataFile;

pub struct FortuneFileInfo {
    pub path: PathBuf,
    pub data_file: DataFile,
}
impl FortuneFileInfo {
    pub fn new(path: PathBuf, data_file: DataFile) -> Self {
        Self { path, data_file }
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn count(&self) -> usize {
        self.data_file.header.count() as usize
    }
    pub fn offset_at(&self, index: usize) -> u32 {
        self.data_file.offsets[index]
    }
    pub fn fortune_file(&self) -> String {
        let file = self.path.with_extension("");
        file.file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_owned()
    }
}
