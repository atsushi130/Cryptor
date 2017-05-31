# Enigma
[![Enigma](https://img.shields.io/badge/Cryptor-Enigma-6fb536.svg)](https://github.com/atsushi130/Cryptor/tree/master/src/cryptor/algorithm/enigma)
[![Document](https://img.shields.io/badge/Enigma-Document-3B5998.svg)](https://docs.rs/cryptor/0.1.4/cryptor/cryptor/struct.Enigma.html)

## Usage
**Import modules**
```rust
extern crate cryptor;
use cryptor::cryptor::{ Enigma, Router, Reflector, Plugboard, SubstitutionTable };
use cryptor::cryptor::{ Cryptor, CryptoValue };
use cryptor::cryptor::{ SUBSTITUTION_TABLE1, SUBSTITUTION_TABLE2, SUBSTITUTION_TABLE3, REFLECTOR, PLUGBOARD };
```

**Setup Enigma**  
Enigma factory method is `Enigma::new(Vec<Router>, Plugboard, Reflector) -> Self`. You can also use customized substitution table by yourself.
```rust
let enigma = Enigma::new(
    vec![
        Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE1.to_vec())),
        Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE2.to_vec())),
        Router::new(SubstitutionTable::new(SUBSTITUTION_TABLE3.to_vec())),
    ],
    Plugboard::new(SubstitutionTable::new(PLUGBOARD.to_vec())),
    Reflector::new(SubstitutionTable::new(REFLECTOR.to_vec()))
);
```

**Setup Cryptor**  
```rust
let mut cryptor = Cryptor::new(enigma);
```

**Encryption**  
```rust
match cryptor.encrypt(&"A quick brown fox jumps over the lazy dog.") {
    Ok(ref crypted) => println!("crypted: {}", crypted.text),
    Err(ref error)  => println!("{}", error)
}
```

**Decryption**  
```rust
match cryptor.decrypt(&string) {
    Ok(ref crypted) => println!("crypted: {}", crypted.text), // crypted: A quick brown fox jumps over the lazy dog.
    Err(ref error)  => println!("{}", error)
}
```

**Set router's position**  
To set the position, specify a character string. The length of the specified string must be equal to the number of routers.
```rust
match cryptor.algorithm.set_positons("ABC") {
    Ok(_)          => println!("set positions."),
    Err(ref error) => println!("{}", error)
}
```
