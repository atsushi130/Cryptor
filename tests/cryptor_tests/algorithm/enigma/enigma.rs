
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
