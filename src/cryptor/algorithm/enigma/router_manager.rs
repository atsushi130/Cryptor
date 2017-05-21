
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

/// enigma module
use super::{ Router, RouterProtocol, Digit };

pub struct RouterManager {
    pub routers: Vec<Router>
}

impl RouterManager {

    pub fn crypto_to_reflector(&self, character: &char) -> char {

        let mut encrypt = *character;
        for router in &self.routers {
            encrypt = router.crypto(&encrypt);
        }

        return encrypt
    }

    pub fn crypto_from_reflector(&self, character: &char) -> char {

        let mut encrypt = *character;
        for router in self.routers.iter().rev() {
            encrypt = router.reverse(&encrypt);
        }

        return encrypt
    }

    pub fn increment(&mut self) {
        for router in &mut self.routers {
            match router.increment() {
                Digit::Up       => continue,
                Digit::NoChange => break
            }
        }
    }
}
