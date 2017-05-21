
use std::marker::PhantomData;

/// algorithm module
use super::Algorithm;

pub struct CryptoValue<T: Algorithm> {
    pub text: String,
    pub phantom: PhantomData<T>
}

impl<T: Algorithm> CryptoValue<T> {
    pub fn new(text: &str) -> Self {
        CryptoValue {
            text: text.to_string(),
            phantom: PhantomData
        }
    }
}
