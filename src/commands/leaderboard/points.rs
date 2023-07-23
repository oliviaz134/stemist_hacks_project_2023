use super::super::pq;
use super::super::super::json_structs::parse;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static QUESTION_RESPONSE: Lazy<Mutex<std::collections::HashMap<i64, Vec<bool>>>> =
    Lazy::new(|| Mutex::new(std::collections::HashMap::new()));

pub fn add_points(id: i64, amount_of_points: i64) {
    let mut connection = pq::connect::establish_connection();

    let mut user = pq::interface::get_user(&mut connection, id).unwrap();

    user.points += amount_of_points;

    pq::interface::user_update_points(&mut connection, id, user.points).unwrap_or_default();
}

pub fn initialize_question_response_first_time() {
    let mut map = QUESTION_RESPONSE.lock().unwrap();
    let mut connection = pq::connect::establish_connection();

    let users = pq::interface::get_all_users(&mut connection);
    let num_of_questions = parse::json_data_size();

    if let Ok(users) = users {
        for user in users {
            map.insert(user.user_id, vec![false; num_of_questions]);
        }
    }

    // Serialize the map to a file
    let serialized = serde_json::to_string(&map).unwrap();
    let mut file = std::fs::File::create("./resources/answered_questions.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}



pub fn mark_question_answered_right(user_id : i64, _question_id : i64) {
    let mut map = QUESTION_RESPONSE.lock().unwrap();

    if map.contains_key(&user_id) {
        map.get_mut(&user_id).unwrap().push(true);
    } else {
        map.insert(user_id, vec![true]);
    }
}

