/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-05-02
 */
use serde::Serialize;

#[derive(Serialize)]
pub struct Fortune {
    file: String,
    fortune: String,
}
impl Fortune {
    pub fn new(file: String, fortune: String) -> Self {
        Self { file, fortune }
    }
    pub fn file(&self) -> String {
        self.file.to_owned()
    }
    pub fn fortune(&self) -> String {
        self.fortune.to_owned()
    }
}
