
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

/// encrypter crate
extern crate cryptor;

#[cfg(test)]
mod router_manager_tests {

    use cryptor::cryptor::{ Router, RouterManager, SubstitutionTable };
    use cryptor::cryptor::{ ALPHABETS };

    #[test]
    fn incrementable() {

        #![allow(unused_mut)]
        let mut router_manager = RouterManager {
            routers: vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ]
        };

        router_manager.increment();

        let expects = vec![1, 0, 0];
        for i in 0..3 {
            assert_eq!(expects[i], router_manager.routers[i].get_position());
        }
    }

    #[test]
    fn incrementable_digit_up() {

        #![allow(unused_mut)]
        let mut router_manager = RouterManager {
            routers: vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ]
        };

        router_manager.routers[0].set_position(&'0');
        router_manager.routers[1].set_position(&'0');
        router_manager.routers[2].set_position(&'9');
        router_manager.increment();

        let expects = vec![0, 0, 95];
        for i in 0..3 {
            assert_eq!(expects[i], router_manager.routers[i].get_position());
        }
    }

    #[test]
    fn encryptable_to_reflector() {

        #![allow(unused_mut)]
        let mut router_manager = RouterManager {
            routers: vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ]
        };

        let result = router_manager.crypto_to_reflector(&'A');
        assert_eq!('A', result);
    }

    #[test]
    fn encryptable_from_reflector() {

        #![allow(unused_mut)]
        let mut router_manager = RouterManager {
            routers: vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ]
        };

        let result = router_manager.crypto_to_reflector(&'A');
        assert_eq!('A', result);
    }

    #[test]
    fn encryptable_to_reflector_after_increment() {

        #![allow(unused_mut)]
        let mut router_manager = RouterManager {
            routers: vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ]
        };

        router_manager.increment();
        let result = router_manager.crypto_to_reflector(&'A');
        assert_eq!('B', result);
    }

    #[test]
    fn encryptable_from_reflector_after_increment() {

        #![allow(unused_mut)]
        let mut router_manager = RouterManager {
            routers: vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ]
        };

        router_manager.increment();
        let result = router_manager.crypto_from_reflector(&'a');
        assert_eq!('0', result);
    }
}
