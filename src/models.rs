use diesel::prelude::*;

#[derive(Queryable, Insertable, Eq, PartialEq, Debug, Selectable)]
#[diesel(table_name = crate::schema::user_information)]
pub struct UserInformation {
    pub user_id: i64,
    pub username: String,
    pub points: i64,
}