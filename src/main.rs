#![feature(slice_group_by)]

mod cli;
mod controllers;
mod model;
mod services;

use std::{env, net::SocketAddr};

use axum::{
    routing::{get, post},
    Extension, Router,
};
use clap::Parser;
use model::error::Error;
use tera::Tera;
use tracing::info;

use controllers::*;
use tracing_subscriber::{prelude::*, EnvFilter};

use crate::{
    cli::Cli,
    controllers::{
        about::{get_admin_about_page, post_about},
        category::{delete_category, get_admin_category_page, move_category, post_category},
        faq::{delete_faq, get_admin_faq_page, move_faq, post_faq},
        image::{
            delete_image, get_admin_edit_image_page, get_admin_images_page, move_image, post_image,
            put_image, hide_image,
        },
    },
    services::{database::Database, static_files::StaticFiles},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    let cli = Cli::parse();

    let root_dir = cli.root_dir.canonicalize()?;

    info!("Starting database...");
    let db = Database::new(&root_dir).await?;
    db.migrate().await?;

    let templates = root_dir.join("templates").display().to_string() + "/*";
    tracing::info!("Using template directory: {}", templates);
    let tera = Tera::new(&templates).unwrap();

    let static_files = StaticFiles::new(cli.root_dir);

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
        .route("/thumbs/:filename", get(serve_thumb))
        .route("/styles/:filename", get(serve_styles))
        // Admin stuff
        .route("/admin", get(get_admin_page))
        .route(
            "/admin/categories",
            get(get_admin_category_page).post(post_category),
        )
        .route("/admin/categories/move", post(move_category))
        .route("/admin/categories/delete", post(delete_category))
        .route("/admin/images", get(get_admin_images_page).post(post_image))
        .route("/admin/images/edit/:image", get(get_admin_edit_image_page))
        .route("/admin/images/delete", post(delete_image))
        .route("/admin/images/update", post(put_image))
        .route("/admin/images/move", post(move_image))
        .route("/admin/images/hide", post(hide_image))
        .route("/admin/about", get(get_admin_about_page).post(post_about))
        .route("/admin/faq", get(get_admin_faq_page).post(post_faq))
        .route("/admin/faq/delete", post(delete_faq))
        .route("/admin/faq/move", post(move_faq))
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

fn setup_tracing() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "jinwonkim_art=debug");
    }

    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::Layer::new().with_writer(std::io::stdout))
        .with(EnvFilter::from_default_env());

    tracing::subscriber::set_global_default(subscriber).expect("Unable to set global subscriber");

    let rust_log = env::var_os("RUST_LOG").unwrap_or("***ENV VAR NOT SET***".into());

    tracing::info!("Setup logging with: {:?}", rust_log);
}
