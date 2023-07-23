use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Options {
    pub a: String,
    pub b: String,
    pub c: String,
    pub d: String
}

#[derive(Debug, Deserialize)]
pub struct Question {
    pub id: i64,
    pub question: String,
    pub options: Options,
    pub answer: String
}
 
#[derive(Debug, Deserialize)]
pub struct Config {
    pub questions: Vec<Question>
}
