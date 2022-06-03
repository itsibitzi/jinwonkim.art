use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use tera::{Context, Tera};
use uuid::Uuid;

use crate::{
    model::forms::image::CreateImage,
    services::{database::Database, image_store::ImageStore},
};

pub async fn get_images(Extension(tera): Extension<Tera>) -> Html<String> {
    let mut ctx = Context::new();
    ctx.insert("where", "images");

    Html(tera.render("images.html", &ctx).unwrap())
}

pub async fn post_image(
    payload: Multipart,
    Extension(image_store): Extension<ImageStore>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let image_upload = CreateImage::from_multipart(payload)
        .await
        .map_err(|e| e.into())?;

    let ext = if image_upload.img_name.ends_with("png") {
        ".png"
    } else if image_upload.img_name.ends_with("jpg") || image_upload.img_name.ends_with("jpeg") {
        ".jpg"
    } else {
        ""
    };

    let mut filename = Uuid::new_v4().to_string();
    filename.push_str(ext);

    image_store
        .save_image(&filename, &image_upload.img)
        .map_err(|e| e.into())?;

    db.create_image(&image_upload.name, &filename)
        .await
        .map(|_| Redirect::to("/admin"))
        .map_err(|e| e.into())
}
