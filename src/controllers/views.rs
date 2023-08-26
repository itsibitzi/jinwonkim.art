use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use tera::{Context, Tera};

use crate::services::{database::Database, static_files::StaticFiles};

pub async fn get_home_page(
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut ctx = Context::new();

    let images = db.list_images().await.map_err(|e| e.into())?;
    let categories = db.list_categories().await.map_err(|e| e.into())?;

    ctx.insert("current_page", "home");
    ctx.insert("categories", &categories);
    ctx.insert("images", &images);

    Ok(Html(tera.render("homepage.html", &ctx).unwrap()))
}

pub async fn get_category_page(
    Path(category): Path<String>,
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut ctx = Context::new();

    let images = db
        .list_images_for_category(&category)
        .await
        .map_err(|e| e.into())?;
    let categories = db.list_categories().await.map_err(|e| e.into())?;

    ctx.insert("current_page", &category);
    ctx.insert("categories", &categories);
    ctx.insert("images", &images);

    Ok(Html(tera.render("categories.html", &ctx).unwrap()))
}
pub async fn get_image_page(
    Path(image): Path<i64>,
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut ctx = Context::new();

    let image = db.get_image_by_id(image).await.map_err(|e| e.into())?;
    let categories = db.list_categories().await.map_err(|e| e.into())?;

    ctx.insert("current_page", "image");
    ctx.insert("categories", &categories);
    ctx.insert("image", &image);

    Ok(Html(tera.render("images.html", &ctx).unwrap()))
}

pub async fn get_about_page(
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut ctx = Context::new();

    let categories = db.list_categories().await.map_err(|e| e.into())?;
    let about = db.select_about().await.map_err(|e| e.into())?;

    ctx.insert("current_page", "about");
    ctx.insert("categories", &categories);
    ctx.insert("about", &about);

    Ok(Html(tera.render("about.html", &ctx).unwrap()))
}

pub async fn get_faq_page(
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut ctx = Context::new();

    let categories = db.list_categories().await.map_err(|e| e.into())?;
    let faqs = db.list_faqs().await.map_err(|e| e.into())?;

    ctx.insert("current_page", "faq");
    ctx.insert("categories", &categories);
    ctx.insert("faqs", &faqs);

    Ok(Html(tera.render("faq.html", &ctx).unwrap()))
}

pub async fn serve_styles(
    Path(filename): Path<String>,
    Extension(static_files): Extension<StaticFiles>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let file = static_files
        .get_style(&filename)
        .await
        .map_err(|e| e.into())?;

    Ok(([(CONTENT_TYPE, "text/css")], file))
}
