
/// algorithm module
use super::EncryptValue;

pub trait Algorithm {
    type V: Algorithm;
    fn encrypt(&mut self, character: &char) -> EncryptValue<Self::V>;
}

