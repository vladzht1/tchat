use std::net::{Ipv4Addr, SocketAddr};

pub type UserId = u32;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct User {
  id: UserId,
  user_name: String,
  address: SocketAddr,
}

impl User {
  pub fn new(id: UserId, user_name: String, address: SocketAddr) -> Self {
    return User {
      id,
      user_name,
      address
    };
  }
}

impl User {
  pub const fn get_id(&self) -> UserId {
    return self.id;
  }

  pub fn get_user_name(&self) -> String {
    return String::from(&self.user_name);
  }
}