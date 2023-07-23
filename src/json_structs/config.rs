#![allow(non_snake_case)]

use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Options {
    pub A: String,
    pub B: String,
    pub C: String,
    pub D: String
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Question {
    pub id: i64,
    pub question: String,
    pub options: Options,
    pub answer: String
}
 
#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    pub questions: Vec<Question>
}

