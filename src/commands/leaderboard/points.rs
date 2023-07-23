use super::super::super::json_structs::parse;
use super::super::pq;
use once_cell::sync::Lazy;
use std::sync::Mutex;

static QUESTION_RESPONSE: Lazy<Mutex<std::collections::HashMap<i64, Vec<bool>>>> =
    Lazy::new(|| Mutex::new(std::collections::HashMap::new()));

pub fn add_points(
    response: serenity::model::application::interaction::message_component::MessageComponentInteraction,
    amount_of_points: i64,
) {
    let mut connection = pq::connect::establish_connection();

    let user_result =
        pq::interface::get_user(&mut connection, response.clone().member.unwrap().user.id.0 as i64);

    let mut user = if let Ok(user) = user_result {
        user
    } else {
        pq::interface::insert_user(
            &mut connection,
            &(response.clone().member.unwrap().user.id.0 as i64),
            response.clone().member.unwrap().user.name.as_str(),
        )
        .unwrap()
    };

    user.points += amount_of_points;
    pq::interface::user_update_points(&mut connection, response.clone().member.unwrap().user.id.0 as i64, user.points).unwrap_or_default();
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
}

pub fn mark_question_answered_right(user_id: i64, _question_id: i64) {
    let mut map = QUESTION_RESPONSE.lock().unwrap();

    if map.contains_key(&user_id) {
        map.get_mut(&user_id).unwrap().push(true);
    } else {
        map.insert(user_id, vec![true]);
    }
}
