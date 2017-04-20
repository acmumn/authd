use schema::users;

#[derive(Clone, Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub realname: Option<String>,
    pub username: String,
    pub passhash: String,
    pub cardhash: Option<String>,
    pub cardlast: Option<String>,
}

#[derive(Clone, Debug, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub email: String,
    pub realname: Option<String>,
    pub username: String,
    pub passhash: String,
    pub cardhash: Option<String>,
    pub cardlast: Option<String>,
}
