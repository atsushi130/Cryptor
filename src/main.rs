
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

#[cfg(not(test))]
fn main() {

    let enigma = Enigma::new(
            vec![
                Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE1.to_vec())),
                Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE2.to_vec())),
                Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE3.to_vec())),
            ],
            Plugboard::new(SubstitutionTable::new(PLUGBOARD.to_vec())),
            Reflector::new(SubstitutionTable::new(REFLECTOR.to_vec()))
        );

    let mut cryptor = Cryptor::new(enigma);

    let characters1 = "0V+/;e.\"%¥HN=P\"%WLkKC=xK[N<(DemmE=+.D\"bErC#X!|^G.{#5:KVr";
    let characters2 = "**~_}*jl\'*fK\'=eG\'\'sP\'\\n<MMY@";

    let crypto_value1: CryptoValue<Enigma> = cryptor.decrypt(&characters1);
    let crypto_value2: CryptoValue<Enigma> = cryptor.decrypt(&characters2);

    println!("crypto: {}", crypto_value1.text);
    println!("crypto: {}", crypto_value2.text);
}
