//! rsw-lib provides libraries needed for rsw-app.
//! # User Factory
//! User factory provides a way to create `n` number of users
//! Example:
//! ```rust
//!        use rsw_lib::device::*;
//!        use rsw_lib::events::*;
//!        use rsw_lib::user::UserFactory;
//!
//!        let capacity = 10;
//!        let request = 5;
//!        let factory = UserFactory::new(capacity)
//!            .create_users(request)
//!            .unwrap();
//!        let res = factory.get_users();
//!        assert_eq!(request, res.len());
//!
//!        let additional = 5;
//!        let factory = factory.create_users(additional).unwrap();
//!        let res = factory.get_users();
//!        assert_eq!(10, res.len());
//! ```
pub mod device;
pub mod events;
pub mod user;

#[cfg(test)]
mod factory_tests {
    use crate::user::UserFactory;

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
