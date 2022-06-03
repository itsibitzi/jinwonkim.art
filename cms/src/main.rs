#![feature(slice_group_by)]

mod controllers;
mod model;
mod services;

use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service, post},
    Extension, Router,
};
use model::error::Error;
use tera::Tera;
use tower_http::services::ServeDir;
use tracing::info;

use controllers::*;

use crate::services::{database::Database, image_store::ImageStore};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    info!("Starting database...");
    let db = Database::new().await?;
    db.migrate().await?;

    let tera = Tera::new("templates/*").unwrap();

    let image_store = ImageStore::new("images");

    info!(
        "Found templates: {}",
        tera.get_template_names().collect::<Vec<&str>>().join(", ")
    );

    let app = Router::new()
        .route("/admin", get(get_admin_page))
        .route("/admin/categories", post(post_category))
        .route("/admin/categories/delete", post(delete_category))
        .route("/admin/images", post(post_image))
        .route("/categories", get(get_categories))
        .route(
            "/images/*",
            get_service(ServeDir::new("images/")).handle_error(handle_error),
        )
        .layer(Extension(tera))
        .layer(Extension(image_store))
        .layer(Extension(db));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("Starting server...");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
