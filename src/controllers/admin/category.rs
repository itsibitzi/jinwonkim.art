use axum::{
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use tera::{Context, Tera};

use crate::{
    model::forms::category::{CreateCategory, DeleteCategory},
    services::{
        auth::{check_password_for_user, AuthBasic},
        database::Database,
    },
};

pub async fn get_admin_category_page(
    AuthBasic((username, password)): AuthBasic,
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        let mut ctx = Context::new();

        let categories = db.list_categories().await.map_err(|e| e.into())?;

        ctx.insert("current_page", "categories");
        ctx.insert("categories", &categories);

        Ok(Html(tera.render("admin_categories.html", &ctx).unwrap()))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}

pub async fn post_category(
    AuthBasic((username, password)): AuthBasic,
    Form(payload): Form<CreateCategory>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        db.create_category(&payload.name)
            .await
            .map(|_| Redirect::to("/admin/categories"))
            .map_err(|e| e.into())
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}

pub async fn delete_category(
    AuthBasic((username, password)): AuthBasic,
    Form(payload): Form<DeleteCategory>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        db.delete_category(&payload.id)
            .await
            .map(|_| Redirect::to("/admin"))
            .map_err(|e| e.into())
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}
