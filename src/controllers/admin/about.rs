use axum::{
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use tera::{Context, Tera};

use crate::{
    model::forms::about::SetAbout,
    services::{
        auth::{check_password_for_user, AuthBasic},
        database::Database,
    },
};

pub async fn get_admin_about_page(
    AuthBasic((username, password)): AuthBasic,
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        let mut ctx = Context::new();

        let about = db.select_about().await.map_err(|e| e.into())?;

        ctx.insert("current_page", "about");
        ctx.insert("about", &about);

        Ok(Html(tera.render("admin_about.html", &ctx).unwrap()))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}

pub async fn post_about(
    AuthBasic((username, password)): AuthBasic,
    Form(payload): Form<SetAbout>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        db.insert_about(payload.about)
            .await
            .map(|_| Redirect::to("/admin/about"))
            .map_err(|e| e.into())
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}
