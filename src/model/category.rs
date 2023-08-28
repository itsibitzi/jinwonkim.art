use serde::Serialize;

#[derive(Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub position: i64,
}

impl Category {
    pub fn into_image_category(self, checked: bool) -> ImageCategory {
        ImageCategory {
            id: self.id,
            name: self.name,
            checked,
        }
    }
}

#[derive(Serialize)]
pub struct ImageCategory {
    pub id: String,
    pub name: String,
    pub checked: bool,
}
