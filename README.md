# Cryptor
[![MIT / Apache2.0 dual licensed](https://img.shields.io/badge/dual%20license-MIT%20/%20Apache%202.0-blue.svg)](./LICENSE-MIT.md)
[![Travis Build Status](https://api.travis-ci.org/atsushi130/Cryptor.svg?branch=master)](https://travis-ci.org/atsushi130/Cryptor)
[![crates.io](https://img.shields.io/crates/v/cryptor.svg)](https://crates.io/crates/cryptor)
[![Document](https://img.shields.io/badge/Cryptor-Document-3B5998.svg)](https://docs.rs/cryptor/0.1.4/cryptor/)

Cryptor is encryption machine corresponding to the diversity of algorithms.

## Dependencies
Insert to Cargo.toml of your project.
```toml
[dependencies]
cryptor = "0.1.4"
```
or
```console
// Newest version
❯ cargo add cryptor

// Version specification
❯ cargo add cryptor@0.1.4

// If not exist on crates.io
❯ mkdir lib
❯ cd lib
❯ git clone https://github.com/atsushi130/Cryptor
❯ cd ..
❯ cargo add cryptor --path=lib/cryptor/
```

## Default crypto algorithm
- [![Enigma](https://img.shields.io/badge/Cryptor-Enigma-6fb536.svg)](https://github.com/atsushi130/Cryptor/tree/master/src/cryptor/algorithm/enigma)
- [![Base64](https://img.shields.io/badge/Cryptor-Base64-6fb536.svg)](https://github.com/atsushi130/Cryptor/tree/master/src/cryptor/algorithm/base64)

## Usage

Import modules
```rust
extern crate cryptor;
use cryptor::cryptor::{ Cryptor, CryptoValue, CryptoError, Algorithm };
```

Implement structure with this Algorithm trait.
```rust
pub trait Algorithm {
    type V: Algorithm;
    fn encrypt(&mut self, character: &char) -> Result<CryptoValue<Self::V>, CryptoError>;
    fn decrypt(&mut self, character: &char) -> Result<CryptoValue<Self::V>, CryptoError>;
}
```

Cryptor have member with Algorithm trait. Dependency injection your implemented structure to Cryptor.
```rust
let mut cryptor = Cryptor::new(YourAlgorithm);
```

Return type of encrypt and decrypt method is `Result<CryptoValue<YourAlgorithm>, CryptoError>`.
```rust
match cryptor.encrypt(&string) {
    Ok(ref crypted) => println!("crypted: {}", crypted.text),
    Err(ref error)  => println!("{}", error)
}

match cryptor.decrypt(&string) {
    Ok(ref crypted) => println!("crypted: {}", crypted.text),
    Err(ref error)  => println!("{}", error)
}
```

## Run
```console
❯ cargo build
❯ cargo run

```

## Test
```console
❯ cargo test

```

## Change logs
**v0.1.4**  
Add exception of Cryptor and Enigma.

**CryptoError**
```rust
pub enum CryptoError {
    InvalidString(String),
    InvalidByte(usize, u8, DecodeError),
    InvalidLength(DecodeError),
    UTF8Error(FromUtf8Error)
}
```

**EnigmaError**
```rust
pub enum EnigmaError {
    InvalidLength
}
```

**changed usage**
```rust
let mut cryptor = Cryptor::new(your_algorithm);
match cryptor.encrypt(string) {
    Ok(ref crypted) => println!("crypted: {}", crypted.text),
    Err(ref error)  => println!("{}", error)
}

match cryptor.decrypt(string) {
    Ok(ref crypted) => println!("crypted: {}", crypted.text),
    Err(ref error)  => println!("{}", error)
}
```

## LICENSE
**This project is dual-licensed under MIT and Apache 2.0.**
