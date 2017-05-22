# Enigma
[![Enigma](https://img.shields.io/badge/Cryptor-Enigma-green.svg)](https://github.com/atsushi130/Cryptor/tree/master/src/cryptor/algorithm/enigma)

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
let mut cryptor = Cryptor { algorithm: enigma };
```

**Encryption**  
```rust
let encrypted: CryptoValue<Enigma> = cryptor.encrypt(&"A quick brown fox jumps over the lazy dog.");
println!("encrypted: {}", encrypted.text);
```

**Decryption**  
```rust
let decrypted: CryptoValue<Enigma> = cryptor.encrypt(&encrypted);
println!("decrypted: {}", decrypted); // decrypted: A quick brown fox jumps over the lazy dog.
```

**Set router's position**  
To set the position, specify a character string. The length of the specified string must be equal to the number of routers.
```
cryptor.algorithm.set_positons("ABC");
```
