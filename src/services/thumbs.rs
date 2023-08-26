use std::path::PathBuf;

use image::imageops::FilterType;
use tracing::{error, info};

pub async fn make_thumbnail(filename: &str) -> anyhow::Result<()> {
    let mut image_path = PathBuf::from("images");
    image_path.push(&filename);
    let full_size_image = image::open(&image_path)?;

    let (nwidth, nheight) = if full_size_image.width() > full_size_image.height() {
        let ratio: f32 = full_size_image.width() as f32 / 400.0;
        let height = full_size_image.height() as f32 / ratio;

        (400, height as u32)
    } else {
        let ratio: f32 = full_size_image.height() as f32 / 400.0;
        let width = full_size_image.width() as f32 / ratio;
        info!(
            "Resizing: {} / {} = {width}",
            full_size_image.height(),
            ratio
        );

        (width as u32, 400)
    };

    let thumb = image::imageops::resize(&full_size_image, nwidth, nheight, FilterType::Lanczos3);

    let mut thumb_path = PathBuf::from("thumbs");
    thumb_path.push(&filename);

    error!("{:?}", thumb_path);
    thumb.save(thumb_path)?;

    Ok(())
}
