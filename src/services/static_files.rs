use std::path::{Path, PathBuf};

use axum::body::Bytes;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::model::error::Error;

#[derive(Clone)]
pub struct StaticFiles {
    image_root: PathBuf,
    thumbs_root: PathBuf,
    styles_root: PathBuf,
}

impl StaticFiles {
    pub fn new() -> Self {
        let image_root = PathBuf::from("images").canonicalize().unwrap();
        let thumbs_root = PathBuf::from("thumbs").canonicalize().unwrap();
        let styles_root = PathBuf::from("styles").canonicalize().unwrap();

        StaticFiles {
            image_root,
            thumbs_root,
            styles_root,
        }
    }

    pub async fn save_image(
        &self,
        file_path: impl AsRef<Path>,
        bytes: &Bytes,
    ) -> Result<(), Error> {
        let file_path = file_path.as_ref();

        let mut path = self.image_root.clone();
        path.push(file_path);

        let canonical = path.parent().unwrap().canonicalize()?;

        println!("{}", canonical.display());
        println!("{}", &self.image_root.display());

        if canonical.starts_with(&self.image_root) {
            let mut file = File::create(path).await?;
            file.write_all(bytes).await?;

            Ok(())
        } else {
            Err(Error::InvalidPath)?
        }
    }

    pub async fn get_file(&self, path: impl AsRef<Path>) -> Result<Vec<u8>, Error> {
        let path = path.as_ref();

        let base = if path.starts_with("images") {
            &self.image_root
        } else if path.starts_with("thumbs") {
            &self.thumbs_root
        } else if path.starts_with("styles") {
            &self.styles_root
        } else {
            Err(Error::InvalidBaseDirectory)?
        };

        let canonical = path.canonicalize()?;

        if canonical.starts_with(base) {
            Ok(tokio::fs::read(&path).await?)
        } else {
            Err(Error::InvalidPath)?
        }
    }
}
