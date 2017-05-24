
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

/// data_encoding module
use base64::{ encode, decode };

/// algorithm module
use super::super::{Algorithm, CryptoValue};

pub struct Base64;

impl Algorithm for Base64 {

    type V = Base64;

    fn encrypt(&mut self, string: &str) -> CryptoValue<Self::V> {
        let bytes   = string.as_bytes();
        let encoded = encode(&bytes);
        CryptoValue::new(&encoded)
    }

    fn decrypt(&mut self, string: &str) -> CryptoValue<Self::V> {
        let bytes = decode(string).unwrap();
        let decoded = String::from_utf8(bytes).unwrap();
        CryptoValue::new(&decoded)
    }
}