use serde::Serialize;

#[derive(Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
}
