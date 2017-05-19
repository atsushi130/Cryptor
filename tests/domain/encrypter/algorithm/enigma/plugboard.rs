
/// encrypter crate
extern crate encrypter;

#[cfg(test)]
mod plugboard_tests {

    use encrypter::domain::{ Plugboard, SubstitutionTable };
    use encrypter::domain::{ ALPHABETS };

    #[test]
    fn inputable() {
        let plugboard = Plugboard::new(SubstitutionTable::new(ALPHABETS.to_vec()));
        let result    = plugboard.input(&'A');

        assert_eq!('A', result);
    }

    #[test]
    fn outputable() {
        let plugboard = Plugboard::new(SubstitutionTable::new(ALPHABETS.to_vec()));
        let result    = plugboard.output(&'A');

        assert_eq!('A', result);
    }
}