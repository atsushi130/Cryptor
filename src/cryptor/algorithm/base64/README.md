# Base64
[![Base64](https://img.shields.io/badge/Cryptor-Base64-6fb536.svg)](https://github.com/atsushi130/Cryptor/tree/master/src/cryptor/algorithm/base64)
[![Document](https://img.shields.io/badge/Base64-Document-3B5998.svg)](https://docs.rs/cryptor/0.1.3/cryptor/cryptor/struct.Base64.html)

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
let encrypted: CryptoValue<Base64> = cryptor.encrypt(&"A quick brown fox jumps over the lazy dog.");
println!("encrypted: {}", encrypted.text);
```

**Decryption**  
```rust
let decrypted: CryptoValue<Base64>> = cryptor.encrypt(&encrypted);
println!("decrypted: {}", decrypted); // decrypted: A quick brown fox jumps over the lazy dog.
```

