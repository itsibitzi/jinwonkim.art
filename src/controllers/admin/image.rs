use axum::{
    extract::{Form, Multipart, Path},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use tera::{Context, Tera};
use uuid::Uuid;

use crate::{
    model::{
        category::ImageCategory,
        forms::image::{
            CreateImage, DeleteImage, HideImage, MoveImage, UpdateImage, UpdateThumbnailCrop,
        },
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
        ctx.insert(
            "max_image_position",
            &images
                .iter()
                .max_by_key(|i| i.position)
                .map(|i| i.position)
                .unwrap_or(i64::MAX),
        );

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
                c.into_image_category(checked)
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

pub async fn get_admin_edit_thumbnail_page(
    AuthBasic((username, password)): AuthBasic,
    Path(image): Path<i64>,
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        let mut ctx = Context::new();

        // TODO seems to be using old method - listing all images then filtering
        // in application code seems silly.
        let images = db.list_images().await.map_err(|e| e.into())?;

        let image = images
            .iter()
            .find(|i| i.id == image)
            .ok_or((StatusCode::NOT_FOUND, "Image not found".to_owned()))?;

        ctx.insert("current_page", "images");
        ctx.insert("image", &image);

        Ok(Html(
            tera.render("admin_edit_image_thumbnail_crop.html", &ctx)
                .unwrap(),
        ))
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

        let uploaded_ext = image_upload.img_name.to_lowercase();

        let ext = if uploaded_ext.ends_with("png") {
            ".png"
        } else if uploaded_ext.ends_with("jpg") || uploaded_ext.ends_with("jpeg") {
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
                tracing::error!("Error while saving image: {}", e);
                (StatusCode::BAD_REQUEST, e.to_string())
            })?;

        make_thumbnail(&filename, image_upload.thumbnail_crop_rect, &static_files)
            .await
            .map_err(|e| {
                tracing::error!("Error while creating thumbnail: {}", e);
                // TODO attempt to clean up saved image
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
        .map_err(|e| {
            // TODO attempt to clean up image and thumbnail
            e.into()
        })
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

pub async fn hide_image(
    AuthBasic((username, password)): AuthBasic,
    Form(payload): Form<HideImage>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        db.hide_image(payload.id, payload.hide)
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

pub async fn post_update_thumbnail_crop(
    AuthBasic((username, password)): AuthBasic,
    Form(payload): Form<UpdateThumbnailCrop>,
    Extension(static_files): Extension<StaticFiles>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        let Some(image) = db.get_image_by_id(payload.id).await.expect("fml") else {
            return Err((StatusCode::NOT_FOUND, "Image not found".to_string()));
        };

        let rect = serde_json::from_str(&payload.thumbnail_crop_rect).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                format!("Invalid rectangle JSON: {}", payload.thumbnail_crop_rect),
            )
        })?;

        make_thumbnail(&image.filename, Some(rect), &static_files)
            .await
            .map_err(|e| {
                tracing::error!("Error while creating thumbnail: {}", e);
                // TODO attempt to clean up saved image
                (StatusCode::BAD_REQUEST, e.to_string())
            })?;

        let mut redirect_path = "/admin/images/edit/".to_string();
        redirect_path.push_str(&payload.id.to_string());

        Ok(Redirect::to(&redirect_path))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}
