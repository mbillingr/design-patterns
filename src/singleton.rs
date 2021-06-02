//! Singleton Pattern
//! =================
//!
//! Rust does not force a constructor convention on the programmers.
//! It's easy to create a private constructor or no constructor at
//! all. Actually, there is nothing special about constructors in
//! Rust - they are normal associated functions, which allows us to
//! put the get_instance logic directly into the constructor.
//! However, it's probably unidiomatic to return &Self from ::new().

use lazy_static::lazy_static;

struct TheSingleton {}

impl TheSingleton {
    /// private constructor
    fn new() -> Self {
        TheSingleton {}
    }

    pub fn get_instance() -> &'static Self {
        lazy_static! {
            static ref INSTANCE: TheSingleton = TheSingleton::new();
        }
        &INSTANCE
    }
}

#[test]
fn singleton_has_only_one_instance() {
    let a = TheSingleton::get_instance();
    let b = TheSingleton::get_instance();
    assert!(std::ptr::eq(a, b));
}
