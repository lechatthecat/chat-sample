use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}