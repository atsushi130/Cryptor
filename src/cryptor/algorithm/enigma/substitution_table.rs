
pub struct SubstitutionTable {
    pub characters: Vec<char>
}

impl SubstitutionTable {
    pub fn new(characters: Vec<char>) -> Self {
        SubstitutionTable {
            characters: characters
        }
    }
}