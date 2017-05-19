
#[macro_use]
extern crate lazy_static;

/// domain module
pub mod domain;
#[allow(unused_imports)]
use domain::{ Enigma, Router, Reflector, Plugboard, SubstitutionTable };
#[allow(unused_imports)]
use domain::{ Encrypter, EncryptValue };
#[allow(unused_imports)]
use domain::{ SUBSTITUTION_TABLE1, SUBSTITUTION_TABLE2, SUBSTITUTION_TABLE3, REFLECTOR, PLUGBOARD };

/// utility module
mod utility;

#[cfg(not(test))]
fn main() {
    let mut encrypter = Encrypter {
        hash: Enigma::new(
            vec![
                Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE1.to_vec())),
                Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE2.to_vec())),
                Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE3.to_vec())),
            ],
            Plugboard::new(SubstitutionTable::new(PLUGBOARD.to_vec())),
            Reflector::new(SubstitutionTable::new(REFLECTOR.to_vec()))
        )
    };

    let characters = "RBUSDGWMG EE CBGAYRV";

    print!("Encrypt string: [ {} => ", characters);
    for character in characters.chars() {
        let encrypted_value: EncryptValue<Enigma> = encrypter.encrypt(&character);
        print!("{}", encrypted_value.text);
    }
    println!(" ]");
}
