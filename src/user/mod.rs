use std::collections::HashMap;
use std::{char::MAX, error::Error, usize};

use crate::{
    device::{Device, DeviceInfo, Platform},
    events::UserEvent,
};
use chrono::prelude::*;

pub type UserId = usize;
pub struct User {
    id: UserId,
    username: String,
    last_login: Option<DateTime<Utc>>,
    is_active: bool,
    active_devices: Vec<DeviceInfo>,
    preferred_plaform: Option<Platform>,
}

pub struct UserFactory {
    users: HashMap<UserId, User>,
    max_capacity: usize,
}

impl UserFactory {
    // associative function
    pub fn new(capacity: usize) -> Self {
        UserFactory {
            users: HashMap::with_capacity(capacity),
            max_capacity: capacity,
        }
    }

    // methods
    pub fn create_users(mut self, req_num: usize) -> Result<Self, String> {
        let current_capacity = self.users.len();
        let can_add: bool = (req_num + current_capacity) <= self.max_capacity;
        if !can_add {
            let msg = format!(
                "only have capacity for {} users.",
                self.max_capacity - current_capacity
            );
            return Err(msg);
        }

        let limit = current_capacity + req_num;
        for id in (current_capacity + 1)..=limit {
            let new_user_id = id;
            let new_user = User {
                id: id,
                username: format!("user{}", id).to_string(),
                last_login: None,
                is_active: false,
                active_devices: Vec::with_capacity(5), // max 5 devices
                preferred_plaform: None,
            };
            self.users.insert(new_user_id, new_user);
        }

        Ok(self)
    }

    pub fn get_users(&self) -> &HashMap<UserId, User> {
        &self.users
    }

    // pub fn update_user(&mut self, user_id: UserId) {
    //     let mut user = self.users.get_mut(index);
    // }
}
