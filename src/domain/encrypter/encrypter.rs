
/// algorithm module
use super::{ Algorithm, EncryptValue };

pub struct Encrypter<T: Algorithm> {
    pub hash: T
}

impl<T: Algorithm> Encrypter<T> {
    pub fn encrypt(&mut self, character: &char) -> EncryptValue<T::V> {
        self.hash.encrypt(character)
    }
}
