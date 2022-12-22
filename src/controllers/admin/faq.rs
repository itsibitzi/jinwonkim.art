use axum::{
    extract::{Form},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use tera::{Context, Tera};

use crate::{
    model::forms::{
        faq::{CreateFaq, DeleteFaq},
    },
    services::{
        auth::{check_password_for_user, AuthBasic},
        database::Database,
    },
};

pub async fn get_admin_faq_page(
    AuthBasic((username, password)): AuthBasic,
    Extension(tera): Extension<Tera>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        let mut ctx = Context::new();

        let images = db.list_faqs().await.map_err(|e| e.into())?;

        ctx.insert("current_page", "faq");
        ctx.insert("faqs", &images);

        Ok(Html(tera.render("admin_faq.html", &ctx).unwrap()))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}

pub async fn post_faq(
    AuthBasic((username, password)): AuthBasic,
    Form(payload): Form<CreateFaq>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        db.create_faq(payload)
            .await
            .map(|_| Redirect::to("/admin/faq"))
            .map_err(|e| e.into())
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}
pub async fn delete_faq(
    AuthBasic((username, password)): AuthBasic,
    Form(payload): Form<DeleteFaq>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        db.delete_faq(payload.id)
            .await
            .map(|_| Redirect::to("/admin/faq"))
            .map_err(|e| e.into())
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}