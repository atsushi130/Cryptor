
/// algorithm module
use super::{ Algorithm, CryptoValue };

pub struct Cryptor<T: Algorithm> {
    pub algorithm: T
}

impl<T: Algorithm> Cryptor<T> {

    pub fn encrypt(&mut self, string: &str) -> CryptoValue<T::V> {
        self.algorithm.encrypt(string)
    }

    pub fn decrypt(&mut self, string: &str) -> CryptoValue<T::V> {
        self.algorithm.decrypt(string)
    }
}
