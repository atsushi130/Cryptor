
/// encrypter crate
extern crate cryptor;

#[cfg(test)]
mod router_tests {

    use cryptor::cryptor::{ Router, RouterProtocol, SubstitutionTable };
    use cryptor::cryptor::{ ALPHABETS };

    #[test]
    fn incrementable() {
        #![allow(unused_mut)]
        let mut router = Router::new(SubstitutionTable::new(ALPHABETS.to_vec()));
        router.increment();

        assert_eq!(1, router.get_position());
    }

    #[test]
    fn encryptable() {

        #![allow(unused_mut)]
        let mut router = Router::new(SubstitutionTable::new(ALPHABETS.to_vec()));
        let result     = router.crypto(&'A');

        assert_eq!('A', result);
    }

    #[test]
    fn encryptable_when_position_equal_one() {

        #![allow(unused_mut)]
        let mut router = Router::new(SubstitutionTable::new(ALPHABETS.to_vec()));

        router.increment();
        let result = router.crypto(&'A');

        assert_eq!('B', result);
    }
}


