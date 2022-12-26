use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateFaq {
    pub question: String,
    pub answer: String,
}

#[derive(Deserialize)]
pub struct DeleteFaq {
    pub id: i64,
}

#[derive(Deserialize)]
pub struct MoveFaq {
    pub id: i64,
    pub up: bool,
}
