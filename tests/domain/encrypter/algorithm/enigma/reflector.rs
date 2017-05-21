
/// encrypter crate
extern crate encrypter;

#[cfg(test)]
mod reflector_tests {

    use encrypter::domain::{ Reflector, SubstitutionTable };
    use encrypter::domain::{ ALPHABETS };

    #[test]
    fn reflectable() {

        let reflector = Reflector::new(SubstitutionTable::new(ALPHABETS.to_vec()));
        let result = reflector.reflect(&'A');

        assert_eq!('A', result);
    }
}