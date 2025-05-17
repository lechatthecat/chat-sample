use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublishRequest {
    pub msg: String,
}