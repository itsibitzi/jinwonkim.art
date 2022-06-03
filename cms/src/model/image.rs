use super::category::Category;

use serde::Serialize;

#[derive(Serialize)]
pub struct Image {
    pub id: i64,
    pub name: String,
    pub filename: String,
    pub categories: Vec<Category>,
}
