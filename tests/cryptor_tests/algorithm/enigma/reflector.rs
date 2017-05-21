
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
