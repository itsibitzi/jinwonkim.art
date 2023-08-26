use axum::{
    extract::{Form, Multipart, Path},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use tera::{Context, Tera};
use tracing::error;
use uuid::Uuid;

use crate::{
    model::{
        category::ImageCategory,
        forms::image::{CreateImage, DeleteImage, MoveImage, UpdateImage},
    },
    services::{
        auth::{check_password_for_user, AuthBasic},
        database::Database,
        static_files::StaticFiles,
        thumbs::make_thumbnail,
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

        ctx.insert("current_page", "images");
        ctx.insert("categories", &categories);
        ctx.insert("images", &images);

        Ok(Html(tera.render("admin_images.html", &ctx).unwrap()))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}

pub async fn get_admin_edit_image_page(
    AuthBasic((username, password)): AuthBasic,
    Path(image): Path<i64>,
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        let mut ctx = Context::new();

        let images = db.list_images().await.map_err(|e| e.into())?;

        let image = images
            .iter()
            .find(|i| i.id == image)
            .ok_or((StatusCode::NOT_FOUND, "Image not found".to_owned()))?;

        let categories: Vec<ImageCategory> = db
            .list_categories()
            .await
            .map_err(|e| e.into())?
            .into_iter()
            .map(|c| {
                let checked = image.categories.iter().any(|ic| ic.id == c.id);
                c.to_image_category(checked)
            })
            .collect();

        ctx.insert("current_page", "images");
        ctx.insert("categories", &categories);
        ctx.insert("image", &image);

        Ok(Html(tera.render("admin_edit_image.html", &ctx).unwrap()))
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

        let ext = if image_upload.img_name.to_lowercase().ends_with("png") {
            ".png"
        } else if image_upload.img_name.to_lowercase().ends_with("jpg")
            || image_upload.img_name.to_lowercase().ends_with("jpeg")
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

        make_thumbnail(&filename).await.map_err(|e| {
            error!("Error while creating thumbnail: {}", e);
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

pub async fn put_image(
    payload: Multipart,
    AuthBasic((username, password)): AuthBasic,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        let image_update = UpdateImage::from_multipart(payload)
            .await
            .map_err(|e| e.into())?;

        db.update_image(
            image_update.id,
            image_update.name,
            image_update.description,
            image_update.categories,
        )
        .await
        .map(|_| Redirect::to("/admin/images"))
        .map_err(|e| e.into())
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}
pub async fn move_image(
    AuthBasic((username, password)): AuthBasic,
    Form(payload): Form<MoveImage>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        db.move_image(payload.id, payload.up)
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
