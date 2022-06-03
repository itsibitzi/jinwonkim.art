use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use tera::{Context, Tera};

use crate::services::database::Database;

pub async fn get_admin_page(
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut ctx = Context::new();

    let images = db.list_images().await.map_err(|e| e.into())?;
    let categories = db.list_categories().await.map_err(|e| e.into())?;

    ctx.insert("categories", &categories);
    ctx.insert("images", &images);

    Ok(Html(tera.render("admin.html", &ctx).unwrap()))
}
