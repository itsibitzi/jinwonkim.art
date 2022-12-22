use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension,
};

use crate::services::{
    auth::{check_password_for_user, AuthBasic},
    database::Database,
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
