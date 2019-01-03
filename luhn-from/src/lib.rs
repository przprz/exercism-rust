extern crate luhn;

use std::fmt::Display;

use luhn::is_valid;

pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        is_valid(&self.code)
    }
}

impl<T: Display> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self {
            code: input.to_string(),
        }
    }
}
