
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

/// algorithm module
use super::CryptoValue;

pub trait Algorithm {
    type V: Algorithm;
    fn encrypt(&mut self, string: &str) -> CryptoValue<Self::V>;
    fn decrypt(&mut self, string: &str) -> CryptoValue<Self::V>;
}

