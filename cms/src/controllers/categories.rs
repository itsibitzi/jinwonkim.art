use axum::{
    extract::Form,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Extension, Json,
};

use crate::{
    model::forms::category::{CreateCategory, DeleteCategory},
    services::database::Database,
};

pub async fn get_categories(
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let categories = db.list_categories().await;

    categories.map(|c| Json(c)).map_err(|e| e.into())
}

pub async fn post_category(
    Form(payload): Form<CreateCategory>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    db.create_category(&payload.name)
        .await
        .map(|_| Redirect::to("/admin"))
        .map_err(|e| e.into())
}

pub async fn delete_category(
    Form(payload): Form<DeleteCategory>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    db.delete_category(&payload.id)
        .await
        .map(|_| Redirect::to("/admin"))
        .map_err(|e| e.into())
}
