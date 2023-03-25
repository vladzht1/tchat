use std::collections::{HashMap, HashSet};

use crate::entities::user::{UserId, User};

pub struct Room {
  id: u32,
  owner: UserId,
  members: HashSet<User>
}

impl Room {
  pub fn get_id(&self) -> u32 {
    return self.id;
  }

  pub fn get_owner(&self) -> UserId {
    return self.owner;
  }
}

impl Room {
  pub fn new(id: u32, owner: UserId) -> Self {
    return Room {
      id,
      owner,
      members: HashSet::new(),
    };
  }

  pub fn insert(&mut self, user: User) -> bool {
    return self.members.insert(user);
  }

  pub fn is_owner(&self, user: User) -> bool {
    return self.owner == user.get_id();
  }
}

pub struct RoomPool {
  pool: HashMap<u32, Room>
}

impl RoomPool {
  pub fn new() -> RoomPool {
    return RoomPool { pool: HashMap::new() };
  }

  pub fn insert(&mut self, id: u32, value: Room) -> bool {
    if self.pool.contains_key(&id) {
      return false;
    }

    return match self.pool.insert(id, value) {
      Some(_) => false,
      None => true
    }
  }

  pub fn exists(&mut self, id: u32) -> bool {
    return self.pool.contains_key(&id);
  }

  pub fn remove(&mut self, id: u32) -> bool {
    if !self.pool.contains_key(&id) {
      return false;
    }

    return match self.pool.remove(&id) {
      Some(_) => true,
      None => false
    }
  }
}
