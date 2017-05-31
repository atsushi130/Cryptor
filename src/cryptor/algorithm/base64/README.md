# Base64
[![Base64](https://img.shields.io/badge/Cryptor-Base64-6fb536.svg)](https://github.com/atsushi130/Cryptor/tree/master/src/cryptor/algorithm/base64)
[![Document](https://img.shields.io/badge/Base64-Document-3B5998.svg)](https://docs.rs/cryptor/0.1.4/cryptor/cryptor/struct.Base64.html)

## Usage
**Import modules**
```rust
extern crate cryptor;
use cryptor::cryptor::{ Base64 };
use cryptor::cryptor::{ Cryptor, CryptoValue };
```

**Setup Cryptor**  
```rust
let mut cryptor = Cryptor::new(Base64);
```

**Encryption**  
```rust
match cryptor.encrypt("A quick brown fox jumps over the lazy dog.") {
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

