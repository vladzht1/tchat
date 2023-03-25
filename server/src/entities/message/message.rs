use crate::entities::user::UserId;

pub struct Message {
  author: UserId,
  text: String,
  room_id: u32
}