use axum::{body::Bytes, extract::Multipart};
use serde::Deserialize;

use crate::model::error::Error;

// Parsed from multipart form data
pub struct CreateImage {
    pub name: String,
    pub description: String,
    pub categories: Vec<String>,
    pub img: Bytes,
    pub img_name: String,
}

impl CreateImage {
    pub async fn from_multipart(mut payload: Multipart) -> Result<CreateImage, Error> {
        let mut name: Option<String> = None;
        let mut description: Option<String> = None;
        let mut categories: Vec<String> = vec![];
        let mut img_name: Option<String> = None;
        let mut img: Option<Bytes> = None;

        while let Some(field) = payload.next_field().await? {
            let field_name = field
                .name()
                .ok_or(Error::IllegalStateError("Missing field name"))?;

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
