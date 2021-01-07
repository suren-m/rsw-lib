use device::Platform;
use events::UserEvent;
use user::UserFactory;

mod device;
mod events;
mod user;

fn gen() {
    let factory = UserFactory::new(10).create_users(5).unwrap();
    let test = factory.get_users();
    let factory = factory.create_users(10).expect("can't add");
    let test_again = factory.get_users();
    println!("done");
}

#[cfg(test)]
mod factory_tests {
    use crate::{gen, user::UserFactory};

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
}
