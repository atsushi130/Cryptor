
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

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
