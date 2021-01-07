use crate::{
    device::{DeviceKind, Platform},
    user::UserId,
};

use chrono::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UserEvent {
    Login(DeviceKind<Platform>, UserId, DateTime<Utc>),
    Logout(DeviceKind<Platform>, UserId, DateTime<Utc>),
}
