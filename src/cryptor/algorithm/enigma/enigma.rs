
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

/// data_encoding module
use base64::{ encode, decode };

/// algorithm module
use super::super::{Algorithm, CryptoValue};

/// enigma module
use super::{ Router, Reflector, RouterManager, Plugboard };

pub struct Enigma {
    router_manager: RouterManager,
    plugboard: Plugboard,
    reflector: Reflector
}

impl Algorithm for Enigma {

    type V = Enigma;

    fn encrypt(&mut self, string: &str) -> CryptoValue<Self::V> {
        let encrypted: Vec<String> = self.encode_base64(string).chars().map( |character| self.crypto(&character)).collect();
        CryptoValue::new(&encrypted.join(""))
    }

    fn decrypt(&mut self, string: &str) -> CryptoValue<Self::V> {
        let decrypted: Vec<String> = string.chars().map( |character| self.crypto(&character)).collect();
        CryptoValue::new(&self.decode_base64(&decrypted.join("")))
    }
}

impl Enigma {

    pub fn new(routers: Vec<Router>, plugboard: Plugboard, reflector: Reflector) -> Self {
        Enigma {
            router_manager: RouterManager {
                routers: routers
            },
            plugboard,
            reflector
        }
    }

    pub fn set_positions(&mut self, positions: &str) -> &Self {

        if positions.chars().count() != self.router_manager.routers.len() {
            panic!("The number of positions does not match the number of routers.");
        }

        for (router, position) in self.router_manager.routers.iter_mut().zip(positions.chars()) {
            router.set_position(&position);
        }

        return self
    }

    fn crypto(&mut self, character: &char) -> String {

        let intput = self.plugboard.input(&character);

        let in_crypted  = self.router_manager.crypto_to_reflector(&intput);
        let reflected   = self.reflector.reflect(&in_crypted);
        let out_crypted = self.router_manager.crypto_from_reflector(&reflected);

        self.router_manager.increment();
        self.plugboard.output(&out_crypted).to_string()
    }

    fn encode_base64(&self, string: &str) -> String {
        let bytes = string.as_bytes();
        encode(&bytes)
    }

    fn decode_base64(&self, string: &str) -> String {
        let bytes = decode(string).unwrap();
        String::from_utf8(bytes).unwrap()
    }
}
