// @generated automatically by Diesel CLI.

diesel::table! {
    user_information (user_id) {
        user_id -> Int8,
        username -> Text,
        points -> Int8,
    }
}
