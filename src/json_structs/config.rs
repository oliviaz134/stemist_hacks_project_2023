use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Question {
    pub id: i8,
    pub question: String
}

#[derive(Debug, Deserialize)]
pub struct Answer {
    pub options: String,
    pub correct_option: String
}