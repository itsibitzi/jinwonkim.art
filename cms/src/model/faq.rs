use serde::Serialize;

#[derive(Serialize)]
pub struct Faq {
    pub id: i64,
    pub question: String,
    pub answer: String,
}