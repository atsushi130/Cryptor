
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

/// enigma module
use super::{ SubstitutionTable, ALPHABETS };

/// utility module
use utility::{ GetIndex, IndexResult };

pub struct Plugboard {
    substitution_table: SubstitutionTable
}

impl Plugboard {

    pub fn new(substitution_table: SubstitutionTable) -> Self {
        Plugboard {
            substitution_table
        }
    }

    pub fn input(&self, character: &char) -> char {
        let characters = ALPHABETS.to_vec();
        match characters.get_index(character) {
            IndexResult::Exist(index) => return self.substitution_table.characters[index],
            IndexResult::None         => *character
        }
    }

    pub fn output(&self, character: &char) -> char {
        let characters = ALPHABETS.to_vec();
        match self.substitution_table.characters.get_index(character) {
            IndexResult::Exist(index) => characters[index],
            IndexResult::None         => *character
        }
    }
}
