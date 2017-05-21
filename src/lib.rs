
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

#[macro_use]
extern crate lazy_static;

extern crate base64;

/// cryptor module
pub mod cryptor;

#[allow(unused_imports)]
use cryptor::{ Enigma, Router, Reflector, Plugboard, SubstitutionTable };
#[allow(unused_imports)]
use cryptor::{ Cryptor, CryptoValue };
#[allow(unused_imports)]
use cryptor::{ SUBSTITUTION_TABLE1, SUBSTITUTION_TABLE2, SUBSTITUTION_TABLE3, REFLECTOR, PLUGBOARD };

/// utility module
mod utility;
