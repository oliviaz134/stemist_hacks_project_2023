use super::super::super::models::UserInformation;
use super::super::super::schema::user_information::dsl::*;
use diesel::prelude::*;

pub fn insert_user(
    conn: &mut PgConnection,
    id_loc: &i64,
    discord_username: &str,
) -> Result<UserInformation, diesel::result::Error> {
    use crate::schema::user_information;

    let new_seaker = UserInformation {
        user_id: *id_loc,
        username: String::from(discord_username),
        points: 0,
    };

    diesel::insert_into(user_information::table)
        .values(&new_seaker)
        .get_result(conn)
}

pub fn user_delete_row(
    conn: &mut PgConnection,
    search: i64
) -> Result<usize, diesel::result::Error> {
    diesel::delete(user_information.filter(user_id.eq(search))).execute(conn)
}

pub fn user_delete_table(conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
    diesel::delete(user_information).execute(conn)
}

pub fn user_update_points(
    conn: &mut PgConnection,
    search: i64,
    new_points: i64,
) -> Result<usize, diesel::result::Error> {
    diesel::update(user_information.filter(user_id.eq(search)))
        .set(points.eq(new_points))
        .execute(conn)
}