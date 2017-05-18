
/// enigma module
use super::{ SubstitutionTable, ALPHABETS };

/// utility module
use utility::{ GetIndex, IndexResult };

pub struct Reflector {
    pub substitution_table: SubstitutionTable
}

impl Reflector {

    pub fn new(substitution_table: SubstitutionTable) -> Self {
        Reflector {
            substitution_table
        }
    }

    pub fn reflect(&self, character: &char) -> char {
        let characters = ALPHABETS.to_vec();
        match characters.get_index(character) {
            IndexResult::Exist(index) => self.substitution_table.characters[index],
            IndexResult::None         => *character
        }
    }
}
