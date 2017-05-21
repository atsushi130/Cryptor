lazy_static! {
    // pub static ref ALPHABETS: Vec<char>           = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    // pub static ref SUBSTITUTION_TABLE1: Vec<char> = "ZYXWVUTSRQPONMLKJIHGFEDCBA".chars().collect();
    // pub static ref SUBSTITUTION_TABLE2: Vec<char> = "BLSXUVWOQGTPMNDZYJRHAFEKCI".chars().collect();
    // pub static ref SUBSTITUTION_TABLE3: Vec<char> = "LBXSVUOWGQPTNMZDJYHRFAKEIC".chars().collect();
    // pub static ref REFLECTOR: Vec<char>           = "BADCFEHGJILKNMPORQTSVUXWZY".chars().collect();
    // pub static ref PLUGBOARD: Vec<char>           = "ZDAFCHEGJLINKPMROTQVSXUYBW".chars().collect();
    pub static ref ALPHABETS: Vec<char>           = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ %=!?,.;:*\"\'`@(){}[]<>#$&~^|¥_/\\+-1234567890".chars().collect();
    pub static ref SUBSTITUTION_TABLE1: Vec<char> = "zyxwvutsrqponmlkjihgfedcbaZYXWVUTSRQPONMLKJIHGFEDCBA 0987654321-+\\_/¥|^~&$#><][}{)(@`\'\"*:;.,?!=%".chars().collect();
    pub static ref SUBSTITUTION_TABLE2: Vec<char> = "blsxuvwoqgtpmndzyjrhafekciBLSXUVWOQGTPMNDZYJRHAFEKCI% !=,?;.*:\'\"@`)(}{][><$#~&|^_¥\\/-+2143658709".chars().collect();
    pub static ref SUBSTITUTION_TABLE3: Vec<char> = "lbxsvuowgqptnmzdjyhrfakeicLBXSVUOWGQPTNMZDJYHRFAKEIC9078563412+-/\\¥_^|&~#$<>[]{}()`@\"\':*.;?,=! %".chars().collect();
    pub static ref REFLECTOR: Vec<char>           = "0987654321-+\\/_¥|^~&$#><][}{)(@`\'\"*:;.,?!=% ZYXWVUTSRQPONMLKJIHGFEDCBAzyxwvutsrqponmlkjihgfedcba".chars().collect();
    pub static ref PLUGBOARD: Vec<char>           = "%=!?,.;:*\"\'`@(){}[]<>#$&~^|¥/_\\+-1234567890 ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars().collect();
}
