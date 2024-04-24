use axum::{body::Bytes, extract::Multipart};
use serde::Deserialize;

use crate::model::error::Error;

#[derive(Deserialize)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// Parsed from multipart form data
pub struct CreateImage {
    pub name: String,
    pub description: String,
    pub categories: Vec<String>,
    pub img: Bytes,
    pub img_name: String,
    pub thumbnail_crop_rect: Option<Rectangle>,
}

impl CreateImage {
    pub async fn from_multipart(mut payload: Multipart) -> Result<CreateImage, Error> {
        let mut name: Option<String> = None;
        let mut description: Option<String> = None;
        let mut categories: Vec<String> = vec![];
        let mut img_name: Option<String> = None;
        let mut img: Option<Bytes> = None;
        let mut thumbnail_crop_rect: Option<Rectangle> = None;

        while let Some(field) = payload.next_field().await? {
            let field_name = field
                .name()
                .ok_or(Error::IllegalStateError("Missing field name"))?;

            tracing::debug!("FOUND FIELD {}", field_name);

            match field_name {
                "name" => {
                    name = Some(field.text().await?);
                }
                "description" => {
                    description = Some(field.text().await?);
                }
                "category" => {
                    let category_id = field.text().await?;
                    categories.push(category_id);
                }
                "img" => {
                    img_name = Some(
                        field
                            .file_name()
                            .ok_or(Error::IllegalStateError("Missing filename on image upload"))?
                            .into(),
                    );
                    img = Some(field.bytes().await?);
                }
                // Optional field
                "thumbnail_crop_rect" => {
                    let crop_rect_json = field.text().await?;
                    thumbnail_crop_rect =
                        serde_json::from_str(&crop_rect_json).map_err(|serde_err| {
                            tracing::error!("Serde error {}", serde_err);
                            Error::IllegalStateError("Malformed thumbnail crop rectangle")
                        })?;
                }
                _ => {}
            }
        }

        match (name, description, img, img_name) {
            (Some(name), Some(description), Some(img), Some(img_name)) => Ok(CreateImage {
                name,
                description,
                categories,
                img,
                img_name,
                thumbnail_crop_rect,
            }),
            _ => Err(Error::IllegalStateError(
                "Missing fields, either name, description or img",
            )),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateImage {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub categories: Vec<String>,
}
impl UpdateImage {
    pub async fn from_multipart(mut payload: Multipart) -> Result<UpdateImage, Error> {
        let mut id: Option<i64> = None;
        let mut name: Option<String> = None;
        let mut description: Option<String> = None;
        let mut categories: Vec<String> = vec![];

        while let Some(field) = payload.next_field().await? {
            let field_name = field
                .name()
                .ok_or(Error::IllegalStateError("Missing field name"))?;

            match field_name {
                "id" => {
                    id = Some(field.text().await?.parse().unwrap());
                }
                "name" => {
                    name = Some(field.text().await?);
                }
                "description" => {
                    description = Some(field.text().await?);
                }
                "category" => {
                    let category_id = field.text().await?;
                    categories.push(category_id);
                }
                _ => {}
            }
        }

        match (id, name, description) {
            (Some(id), Some(name), Some(description)) => Ok(UpdateImage {
                id,
                name,
                description,
                categories,
            }),
            _ => Err(Error::IllegalStateError(
                "Missing fields, either name, description or img",
            )),
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteImage {
    pub id: i64,
}

#[derive(Deserialize)]
pub struct MoveImage {
    pub id: i64,
    pub up: bool,
}

#[derive(Deserialize)]
pub struct HideImage {
    pub id: i64,
    pub hide: bool,
}

#[derive(Deserialize)]
pub struct UpdateThumbnailCrop {
    pub id: i64,
    pub thumbnail_crop_rect: String,
}
