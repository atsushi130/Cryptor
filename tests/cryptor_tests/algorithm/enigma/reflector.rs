
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

/// encrypter crate
extern crate cryptor;

#[cfg(test)]
mod reflector_tests {

    use cryptor::cryptor::{ Reflector, SubstitutionTable };
    use cryptor::cryptor::{ ALPHABETS };

    #[test]
    fn reflectable() {

        let reflector = Reflector::new(SubstitutionTable::new(ALPHABETS.to_vec()));
        let result = reflector.reflect(&'A');

        assert_eq!('A', result);
    }
}
