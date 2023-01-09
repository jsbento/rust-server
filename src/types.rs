#[derive(serde::Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
}