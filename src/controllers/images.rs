use std::path::PathBuf;

use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, StatusCode},
    response::IntoResponse,
    Extension,
};

use crate::services::static_files::StaticFiles;

pub async fn serve_image(
    Path(filename): Path<String>,
    Extension(static_files): Extension<StaticFiles>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut path = PathBuf::from("images");
    path.push(&filename);

    let file = static_files.get_file(&path).await.map_err(|e| e.into())?;
    Ok(([(CONTENT_TYPE, "image/jpeg")], file))
}
