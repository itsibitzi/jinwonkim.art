use serde::Deserialize;

#[derive(Deserialize)]
pub struct SetAbout {
    pub about: String,
}
