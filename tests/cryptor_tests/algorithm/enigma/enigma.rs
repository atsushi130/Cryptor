
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

/// encrypter crate
extern crate cryptor;

#[cfg(test)]
mod enigma_tests {

    use cryptor::cryptor::{ Enigma, Algorithm, Router, Reflector, CryptoValue, Plugboard, SubstitutionTable };
    use cryptor::cryptor::{ ALPHABETS };

    #[test]
    fn encryptable() {

        #![allow(unused_mut)]
        let mut enigma = Enigma::new(
            vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ],
            Plugboard::new(SubstitutionTable::new(ALPHABETS.to_vec())),
            Reflector::new(SubstitutionTable::new(ALPHABETS.to_vec()))
        );

        let result: CryptoValue<Enigma> = enigma.encrypt(&"A");
        assert_eq!("QQ==", result.text);
    }

    #[test]
    fn encryptable_two_characters() {

        #![allow(unused_mut)]
        let mut enigma = Enigma::new(
            vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ],
            Plugboard::new(SubstitutionTable::new(ALPHABETS.to_vec())),
            Reflector::new(SubstitutionTable::new(ALPHABETS.to_vec()))
        );

        let result1: CryptoValue<Enigma> = enigma.encrypt(&"A");
        let result2: CryptoValue<Enigma> = enigma.encrypt(&"A");

        assert_eq!("QQ==", result1.text);
        assert_eq!("QQ==", result2.text);
    }
}
