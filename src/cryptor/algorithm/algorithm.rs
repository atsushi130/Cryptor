
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

/// algorithm module
use super::{ CryptoValue, CryptoError };

pub trait Algorithm {
    type V: Algorithm;
    fn encrypt(&mut self, string: &str) -> Result<CryptoValue<Self::V>, CryptoError>;
    fn decrypt(&mut self, string: &str) -> Result<CryptoValue<Self::V>, CryptoError>;
}
