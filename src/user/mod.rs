use std::collections::HashMap;
use std::{char::MAX, error::Error, usize};

use crate::{
    device::{Device, DeviceInfo, Platform},
    events::UserEvent,
};
use chrono::prelude::*;

pub type UserId = usize;

#[derive(Debug, PartialEq, Clone)]
pub struct User {
    id: UserId,
    username: String,
    last_login: Option<DateTime<Utc>>,
    is_active: bool,
    //active_devices: Vec<DeviceInfo>,
    preferred_plaform: Option<Platform>,
}

impl User {
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn get_preferred_platform(&self) -> Option<Platform> {
        match self.preferred_plaform {
            Some(Platform::IOS) => Some(Platform::IOS),
            Some(Platform::Android) => Some(Platform::Android),
            _ => None,
        }
    }
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
                //active_devices: Vec::with_capacity(5), // max 5 devices
                preferred_plaform: None,
            };
            self.users.insert(new_user_id, new_user);
        }

        Ok(self)
    }

    pub fn get_users(&self) -> &HashMap<UserId, User> {
        &self.users
    }

    pub fn get_user(&self, user_id: UserId) -> Option<&User> {
        self.users.get(&user_id)
    }

    /// update event for users
    /// ```rust
    /// use chrono::Utc;
    ///
    /// use rsw_lib::{
    ///     device::{DeviceKind, Platform},
    ///     events::UserEvent,
    ///     user::UserFactory,
    /// };
    /// let mut factory = UserFactory::new(10).create_users(5).unwrap();   
    /// let kind = DeviceKind::SmartPhone(Platform::IOS);
    /// let ev = UserEvent::Login(kind, 3, Utc::now());
    /// factory.update_event(ev);
    /// ```
    pub fn update_event(&mut self, event: UserEvent) {
        match event {
            UserEvent::Login(device_kind, user_id, timestamp) => {
                let mut user = self.users.get_mut(&user_id).unwrap();
                user.is_active = true;
                user.last_login = Some(timestamp);
                match device_kind {
                    crate::device::DeviceKind::SmartPhone(Platform::Android) => {
                        user.preferred_plaform = Some(Platform::Android);
                    }
                    crate::device::DeviceKind::SmartPhone(Platform::IOS) => {
                        user.preferred_plaform = Some(Platform::IOS);
                    }
                    _ => {
                        println!("not done yet");
                    }
                }
            }
            UserEvent::Logout(device, user_id, timestamp) => {
                let mut user = self.users.get_mut(&user_id).unwrap();
                user.is_active = false;
                user.last_login = Some(timestamp);
            }
        }
    }
}
