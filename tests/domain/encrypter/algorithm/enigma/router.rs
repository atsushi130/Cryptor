
/// encrypter crate
extern crate encrypter;

#[cfg(test)]
mod router_tests {

    use encrypter::domain::{ Router, RouterProtocol, SubstitutionTable };
    use encrypter::domain::{ ALPHABETS };

    #[test]
    fn incrementable() {
        #![allow(unused_mut)]
        let mut router = Router::new(SubstitutionTable::new(ALPHABETS.to_vec()));
        router.increment();

        assert_eq!(1, router.position);
    }

    #[test]
    fn encryptable() {

        #![allow(unused_mut)]
        let mut router = Router::new(SubstitutionTable::new(ALPHABETS.to_vec()));
        let result     = router.encrypt(&'A');

        assert_eq!('A', result);
    }

    #[test]
    fn encryptable_when_position_equal_one() {

        #![allow(unused_mut)]
        let mut router = Router::new(SubstitutionTable::new(ALPHABETS.to_vec()));

        router.increment();
        let result = router.encrypt(&'A');

        assert_eq!('B', result);
    }
}


