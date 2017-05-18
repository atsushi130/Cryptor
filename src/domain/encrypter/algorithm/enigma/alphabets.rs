lazy_static! {
    pub static ref ALPHABETS: Vec<char>           = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    pub static ref SUBSTITUTION_TABLE1: Vec<char> = "ZYXWVUTSRQPONMLKJIHGFEDCBA".chars().collect();
    pub static ref SUBSTITUTION_TABLE2: Vec<char> = "BLSXUVWOQGTPMNDZYJRHAFEKCI".chars().collect();
    pub static ref SUBSTITUTION_TABLE3: Vec<char> = "LBXSVUOWGQPTNMZDJYHRFAKEIC".chars().collect();
    pub static ref REFLECTOR: Vec<char>           = "BADCFEHGJILKNMPORQTSVUXWZY".chars().collect();
    pub static ref PLUGBOARD: Vec<char>           = "ZDAFCHEGJLINKPMROTQVSXUYBW".chars().collect();
}