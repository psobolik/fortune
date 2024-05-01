/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-05-03
 */
use serde::Serialize;

#[derive(Serialize)]
pub struct FortuneStats {
    file: String,
    count: usize,
}
impl FortuneStats {
    pub fn new(file: String, count: usize) -> Self {
        Self { file, count }
    }
    pub fn file(&self) -> String {
        self.file.to_owned()
    }
    pub fn count(&self) -> usize {
        self.count
    }
}
