#![feature(slice_group_by)]

mod controllers;
mod model;
mod services;

use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use model::error::Error;
use tera::Tera;
use tracing::info;

use controllers::*;

use crate::{
    controllers::{
        about::{get_admin_about_page, post_about},
        category::{delete_category, get_admin_category_page, post_category},
        faq::{delete_faq, get_admin_faq_page, post_faq},
        image::{delete_image, get_admin_images_page, post_image},
    },
    services::{database::Database, static_files::StaticFiles},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    info!("Starting database...");
    let db = Database::new().await?;
    db.migrate().await?;

    let tera = Tera::new("templates/*").unwrap();

    let static_files = StaticFiles::new();

    info!(
        "Found templates: {}",
        tera.get_template_names().collect::<Vec<&str>>().join(", ")
    );

    let app = Router::new()
        // Normal
        .route("/", get(get_home_page))
        .route("/faq", get(get_faq_page))
        .route("/about", get(get_about_page))
        .route("/categories/:category", get(get_category_page))
        .route("/art/:image", get(get_image_page))
        .route("/assets/:filename", get(serve_image))
        .route("/styles/:filename", get(serve_styles))
        // Admin stuff
        .route("/admin", get(get_admin_page))
        .route(
            "/admin/categories",
            get(get_admin_category_page).post(post_category),
        )
        .route("/admin/categories/delete", post(delete_category))
        .route("/admin/images", get(get_admin_images_page).post(post_image))
        .route("/admin/images/delete", post(delete_image))
        .route("/admin/about", get(get_admin_about_page).post(post_about))
        .route("/admin/faq", get(get_admin_faq_page).post(post_faq))
        .route("/admin/faq/delete", post(delete_faq))
        .layer(Extension(tera))
        .layer(Extension(static_files))
        .layer(Extension(db));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("Starting server...");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
