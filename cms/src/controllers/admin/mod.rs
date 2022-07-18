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
    model::forms::{
        category::{CreateCategory, DeleteCategory},
        faq::{CreateFaq, DeleteFaq},
        image::CreateImage,
    },
    services::{
        auth::{check_password_for_user, AuthBasic},
        database::Database,
        static_files::StaticFiles,
    },
};

pub mod about;
pub mod category;
pub mod faq;
pub mod image;

pub async fn get_admin_page(
    AuthBasic((username, password)): AuthBasic,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_password_for_user(&username, &password, &db).await {
        Ok(Redirect::to("/admin/categories"))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Failed to check password".into()))
    }
}
