use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use crate::model::error::Error;
use axum::body::Bytes;

#[derive(Clone)]
pub struct ImageStore {
    pub base_dir: PathBuf,
}

impl ImageStore {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> ImageStore {
        ImageStore {
            base_dir: base_dir.as_ref().into(),
        }
    }

    pub fn save_image(&self, file_name: &str, bytes: &Bytes) -> Result<(), Error> {
        let mut path = self.base_dir.clone();
        path.push(file_name);

        let mut file = File::create(path.clone())?;
        file.write_all(bytes)?;

        Ok(())
    }
}
