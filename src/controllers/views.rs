use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use tera::{Context, Tera};

use crate::services::{database::Database, static_files::StaticFiles};

// Context needed by header.html, shared by every public page
async fn header_context(
    db: &Database,
    current_page: &str,
) -> Result<Context, (StatusCode, String)> {
    let mut ctx = Context::new();

    let categories = db.list_categories().await.map_err(|e| e.into())?;
    let has_faqs = !db.list_faqs().await.map_err(|e| e.into())?.is_empty();

    ctx.insert("current_page", current_page);
    ctx.insert("categories", &categories);
    ctx.insert("has_faqs", &has_faqs);

    Ok(ctx)
}

pub async fn get_home_page(
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut ctx = header_context(&db, "home").await?;

    let images = db
        .list_images()
        .await
        .map_err(|e| e.into())?
        .into_iter()
        .filter(|i| !i.hide_on_homepage)
        .collect::<Vec<_>>();

    ctx.insert("images", &images);

    Ok(Html(tera.render("homepage.html", &ctx).unwrap()))
}

pub async fn get_category_page(
    Path(category): Path<String>,
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut ctx = header_context(&db, &category).await?;

    let images = db
        .list_images_for_category(&category)
        .await
        .map_err(|e| e.into())?;

    ctx.insert("images", &images);

    Ok(Html(tera.render("categories.html", &ctx).unwrap()))
}
pub async fn get_image_page(
    Path(image): Path<i64>,
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut ctx = header_context(&db, "image").await?;

    let image = db.get_image_by_id(image).await.map_err(|e| e.into())?;

    ctx.insert("image", &image);

    Ok(Html(tera.render("images.html", &ctx).unwrap()))
}

pub async fn get_about_page(
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut ctx = header_context(&db, "about").await?;

    let about = db.select_about().await.map_err(|e| e.into())?;
    let about = markdown::to_html(&about);

    ctx.insert("about", &about);

    Ok(Html(tera.render("about.html", &ctx).unwrap()))
}

pub async fn get_faq_page(
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut ctx = header_context(&db, "faq").await?;

    let mut faqs = db.list_faqs().await.map_err(|e| e.into())?;
    let faqs = faqs
        .iter_mut()
        .map(|faq| {
            faq.answer = markdown::to_html(&faq.answer);
            faq
        })
        .collect::<Vec<_>>();

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

pub async fn serve_js(
    Path(filename): Path<String>,
    Extension(static_files): Extension<StaticFiles>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let file = static_files.get_js(&filename).await.map_err(|e| e.into())?;

    Ok(([(CONTENT_TYPE, "text/javascript")], file))
}
