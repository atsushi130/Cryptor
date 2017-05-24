
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

/// algorithm module
use super::{ Algorithm, CryptoValue };

pub struct Cryptor<T: Algorithm> {
    pub algorithm: T
}

impl<T: Algorithm> Cryptor<T> {

    pub fn new(algorithm: T) -> Self {
        Cryptor {
            algorithm
        }
    }

    pub fn encrypt(&mut self, string: &str) -> CryptoValue<T::V> {
        self.algorithm.encrypt(string)
    }

    pub fn decrypt(&mut self, string: &str) -> CryptoValue<T::V> {
        self.algorithm.decrypt(string)
    }
}
