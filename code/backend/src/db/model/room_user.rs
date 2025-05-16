use serde::{Serialize, Deserialize};

use crate::db::model::user::User;
use crate::db::model::room::Room;

#[derive(Serialize, Deserialize, Debug)]
pub struct RoomUser {
    pub room: Room,
    pub user: User,
    pub updated_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}
