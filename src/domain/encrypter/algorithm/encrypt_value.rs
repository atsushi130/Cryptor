
use std::marker::PhantomData;

/// algorithm module
use super::Algorithm;

pub struct EncryptValue<T: Algorithm> {
    pub text: String,
    pub phantom: PhantomData<T>
}
