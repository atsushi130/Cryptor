
/// encrypter crate
extern crate encrypter;

#[cfg(test)]
mod enigma_tests {

    use encrypter::domain::{ Enigma, Algorithm, Router, Reflector, EncryptValue, Plugboard, SubstitutionTable };
    use encrypter::domain::{ ALPHABETS };

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

        let result: EncryptValue<Enigma> = enigma.encrypt(&'A');
        assert_eq!("A", result.text);
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

        let result1: EncryptValue<Enigma> = enigma.encrypt(&'A');
        let result2: EncryptValue<Enigma> = enigma.encrypt(&'A');

        assert_eq!("A", result1.text);
        assert_eq!("A", result2.text);
    }
}