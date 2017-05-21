
/// algorithm module
use super::CryptoValue;

pub trait Algorithm {
    type V: Algorithm;
    fn encrypt(&mut self, string: &str) -> CryptoValue<Self::V>;
    fn decrypt(&mut self, string: &str) -> CryptoValue<Self::V>;
}

