use axum::{
    extract::{Form, Multipart},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use tera::{Context, Tera};
use tracing::error;
use uuid::Uuid;

use crate::{
    model::forms::image::{CreateImage, DeleteImage},
    services::{
        auth::{check_password_for_user, AuthBasic},
        database::Database,
        static_files::StaticFiles,
    },
};

pub async fn get_admin_images_page(
    AuthBasic((username, password)): AuthBasic,
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        let mut ctx = Context::new();

        let images = db.list_images().await.map_err(|e| e.into())?;
        let categories = db.list_categories().await.map_err(|e| e.into())?;

        ctx.insert("categories", &categories);
        ctx.insert("images", &images);

        Ok(Html(tera.render("admin_images.html", &ctx).unwrap()))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}

pub async fn post_image(
    payload: Multipart,
    AuthBasic((username, password)): AuthBasic,
    Extension(static_files): Extension<StaticFiles>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        let image_upload = CreateImage::from_multipart(payload)
            .await
            .map_err(|e| e.into())?;

        let ext = if image_upload.img_name.ends_with("png") {
            ".png"
        } else if image_upload.img_name.ends_with("jpg") || image_upload.img_name.ends_with("jpeg")
        {
            ".jpg"
        } else {
            ""
        };

        let mut filename = Uuid::new_v4().to_string();
        filename.push_str(ext);

        static_files
            .save_image(&filename, &image_upload.img)
            .await
            .map_err(|e| {
                error!("Error while saving image: {}", e);
                (StatusCode::BAD_REQUEST, e.to_string())
            })?;

        db.create_image(
            image_upload.name,
            image_upload.description,
            filename,
            image_upload.categories,
        )
        .await
        .map(|_| Redirect::to("/admin/images"))
        .map_err(|e| e.into())
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}

pub async fn delete_image(
    AuthBasic((username, password)): AuthBasic,
    Form(payload): Form<DeleteImage>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        db.delete_image(payload.id)
            .await
            .map(|_| Redirect::to("/admin/images"))
            .map_err(|e| e.into())
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}
