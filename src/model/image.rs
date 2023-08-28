use super::category::Category;

use serde::Serialize;

#[derive(Serialize)]
pub struct Image {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub filename: String,
    pub categories: Vec<Category>,
    pub position: i64,
    pub hide_on_homepage: bool,
}
