use std::ops::Deref;
use rand::prelude::*;
use std::convert::TryFrom;
use std::cell::RefCell;

/// EmailString type that validates and holds a valid email address.
#[derive(Debug, PartialEq)]
pub struct EmailString(String);

impl TryFrom<&str> for EmailString {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains('@') && value.contains('.') {
            Ok(EmailString(value.to_string()))
        } else {
            Err("Invalid email format")
        }
    }
}

impl Deref for EmailString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Random<T> type that holds three values and returns one randomly on dereference.
pub struct Random<T> {
    values: [T; 3],
    rng: RefCell<ThreadRng>,
}

impl<T> Random<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Random {
            values: [a, b, c],
            rng: RefCell::new(thread_rng()),
        }
    }
}

impl<T> Deref for Random<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let mut rng = self.rng.borrow_mut();
        let index = rng.gen_range(0..3);
        &self.values[index]
    }
}

impl<T: Clone> From<[T; 3]> for Random<T> {
    fn from(array: [T; 3]) -> Self {
        Random::new(array[0].clone(), array[1].clone(), array[2].clone())
    }
}

/// Unit tests for EmailString and Random<T>.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_string_valid() {
        let email = EmailString::try_from("test@example.com");
        assert!(email.is_ok());
        assert_eq!(email.unwrap().as_str(), "test@example.com");
    }

    #[test]
    fn test_email_string_invalid() {
        let email = EmailString::try_from("invalidemail");
        assert!(email.is_err());
    }

    #[test]
    fn test_random_usage() {
        let random_values = Random::new(1, 2, 3);
        let val = *random_values;
        assert!(val == 1 || val == 2 || val == 3);
    }

    #[test]
    fn test_random_from_array() {
        let random_values = Random::from([10, 20, 30]);
        let val = *random_values;
        assert!(val == 10 || val == 20 || val == 30);
    }
}
