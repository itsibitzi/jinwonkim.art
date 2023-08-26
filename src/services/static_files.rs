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
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        let image_root = root_dir.as_ref().join("images").canonicalize().unwrap();
        let thumbs_root = root_dir.as_ref().join("thumbs").canonicalize().unwrap();
        let styles_root = root_dir.as_ref().join("styles").canonicalize().unwrap();

        tracing::info!("Using images root: {}", image_root.display());
        tracing::info!("Using thumbs root: {}", thumbs_root.display());
        tracing::info!("Using styles root: {}", styles_root.display());

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

        if canonical.starts_with(&self.image_root) {
            tracing::info!("Saving image: {}", path.display());

            let mut file = File::create(path).await?;
            file.write_all(bytes).await?;

            Ok(())
        } else {
            Err(Error::InvalidPath)?
        }
    }

    pub async fn get_image(&self, name: &str) -> Result<Vec<u8>, Error> {
        let path = self.image_root.join(name);

        tracing::info!("Loading image: {}", path.display());

        Ok(tokio::fs::read(&path).await?)
    }

    pub async fn get_thumb(&self, name: &str) -> Result<Vec<u8>, Error> {
        let path = self.thumbs_root.join(name);

        tracing::info!("Loading thumb: {}", path.display());

        Ok(tokio::fs::read(&path).await?)
    }

    pub async fn get_style(&self, name: &str) -> Result<Vec<u8>, Error> {
        let path = self.styles_root.join(name);

        tracing::info!("Loading style: {}", path.display());

        Ok(tokio::fs::read(&path).await?)
    }
}
