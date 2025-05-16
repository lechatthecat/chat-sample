use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
   pub id: i32,
   pub name: String,
   pub updated_at: chrono::NaiveDateTime,
   pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct UserData {
   pub id: i32,
   pub name: String,
}
