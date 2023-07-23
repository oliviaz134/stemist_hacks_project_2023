#![allow(unused_imports)]

use core::arch::x86_64::_rdrand32_step;
use once_cell::sync::Lazy;
use std::fs;
use std::sync::Mutex;

use super::config::*;

static JSON_DATA: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::default()));

pub fn parse_json_questions() {
    let configuration_file = fs::read_to_string("./resources/questions_and_answers.json")
        .expect("Should have been able to read the file");
    let json: Config =
        serde_json::from_str(&configuration_file).expect("JSON was not well-formatted");

    *JSON_DATA.lock().unwrap() = json;
}

pub fn generate_question() -> Question {
    let mut random = 0;

    unsafe {
        _rdrand32_step(&mut random);
    }

    for question in &JSON_DATA.lock().unwrap().questions {
        if question.id == (random % 61) as i64 {
            return question.clone();
        }
    }

    return Question::default();
}

pub fn check_question(question_id: i64, choice: String) -> bool {
    for question in &JSON_DATA.lock().unwrap().questions {
        if question.id == question_id {
            if question.answer == choice {
                return true;
            }
        }
    }

    return false;
}