use std::ops::{Add, Deref};
use std::fmt;

#[derive(Clone)]
pub struct String_(String);

impl Deref for String_ {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<String_> for String {
    fn into(self) -> String_ {
        String_(self)
    }
}

impl Add for String_ {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        String_ { 0: self.0 + &rhs.0 }
    }
}

impl fmt::Display for String_ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        String::fmt(&self.0, f)
    }
}