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
