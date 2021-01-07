use chrono::Utc;

use rsw_lib::{
    device::{DeviceKind, Platform},
    events::UserEvent,
    user::UserFactory,
};

mod common;

#[test]
fn ensure_user_creation() {
    let capacity = 10;
    let initial_request = 5;
    let factory = UserFactory::new(capacity)
        .create_users(initial_request)
        .unwrap();
    let res = factory.get_users();
    assert_eq!(5, res.len());

    let additional = 5;
    let factory = factory.create_users(additional).unwrap();
    let res = factory.get_users();
    assert_eq!(10, res.len());
}

#[test]
fn fail_when_requested_more_than_capacity() {
    let capacity = 10;
    let initial_request = 15;
    let res = UserFactory::new(capacity)
        .create_users(initial_request)
        .is_err();
    assert!(res);
}

#[test]
fn get_after_update() {
    let capacity = 10;
    let initial_request = 5;
    let mut factory = UserFactory::new(capacity)
        .create_users(initial_request)
        .unwrap();

    let kind = DeviceKind::SmartPhone(Platform::IOS);
    let ev = UserEvent::Login(kind, 3, Utc::now());
    factory.update_event(ev);

    let res = factory.get_user(3).unwrap();
    assert_eq!(res.is_active(), true);
    assert_eq!(res.get_preferred_platform(), Some(Platform::IOS));

    let users = factory.get_users();
    println!("done");
}

#[test]
fn device_info_clone_test() {
    let dev_kind = DeviceKind::SmartPhone(Platform::Android);
    let dev_kind2 = dev_kind.clone();
    assert_eq!(dev_kind, dev_kind2);
}
