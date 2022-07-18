use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use crate::services::database::Database;

pub async fn get_categories(
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let categories = db.list_categories().await;

    categories.map(|c| Json(c)).map_err(|e| e.into())
}
