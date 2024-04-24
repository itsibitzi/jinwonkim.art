use image::imageops::FilterType;

use crate::model::forms::image::Rectangle;

use super::static_files::StaticFiles;

// Images use positive integers ONLY,
// the cropping library can return double
// precision floating point values
struct ImageRectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl From<Rectangle> for ImageRectangle {
    fn from(value: Rectangle) -> Self {
        Self {
            x: value.x as u32,
            y: value.y as u32,
            width: value.width as u32,
            height: value.height as u32,
        }
    }
}

pub async fn make_thumbnail(
    filename: &str,
    crop_rect: Option<Rectangle>,
    static_files: &StaticFiles,
) -> anyhow::Result<()> {
    let image_path = static_files.get_image_path(filename);

    let mut image = image::open(&image_path)?;

    tracing::debug!("Successfully loaded full size image {}", filename);

    if let Some(rect) = crop_rect {
        let ImageRectangle {
            x,
            y,
            width,
            height,
        } = ImageRectangle::from(rect);

        image = image.crop(x, y, width, height);
        tracing::debug!("Cropped image {}", filename);
    }

    let (nwidth, nheight) = if image.width() > image.height() {
        let ratio = image.width() as f32 / 400.0;
        let height = image.height() as f32 / ratio;

        (400, height as u32)
    } else {
        let ratio = image.height() as f32 / 400.0;
        let width = image.width() as f32 / ratio;

        tracing::info!("Resizing: {} / {} = {width}", image.height(), ratio);

        (width as u32, 400)
    };

    tracing::debug!("Resizing image {}", filename);
    let thumb = image::imageops::resize(&image, nwidth, nheight, FilterType::Lanczos3);
    tracing::debug!("Successfully resized image {}", filename);

    static_files.save_thumb(filename, &thumb)?;

    Ok(())
}
