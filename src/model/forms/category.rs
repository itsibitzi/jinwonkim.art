use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateCategory {
    pub name: String,
}

#[derive(Deserialize)]
pub struct DeleteCategory {
    pub id: String,
}
