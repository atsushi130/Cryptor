# Cryptor
Cryptor is encryption machine corresponding to the diversity of algorithms.

## Dependencies
Insert to Cargo.toml of your project.
```toml
[dependencies]
cryptor = "0.1.0"
```
or
```console
// Newest version
❯ cargo add cryptor

// Version specification
❯ cargo add cryptor@0.1.0

// If not exist on crates.io
❯ mkdir lib
❯ cd lib
❯ git clone https://github.com/atsushi130/Cryptor
❯ cd ..
❯ cargo add cryptor --path=lib/cryptor/
```

## Usage
Implement structure with this Algorithm trait.
```rust
pub trait Algorithm {
    type V: Algorithm;
    fn encrypt(&mut self, character: &char) -> CryptoValue<Self::V>;
    fn decrypt(&mut self, character: &char) -> CryptoValue<Self::V>;
}
```

Cryptor have member with Algorithm trait. Dependency injection your implemented structure to Cryptor.
```rust
let mut cryptor = Cryptor {
    algorithm: YourAlgorithm { ... }
};
```

Return type of encrypt and decrypt method is `CryptoValue<YourAlgorithm>`;
```rust
let encrypted: CryptoValue<YourAlgorithm> = cryptor.encrypt(&string);
println!("encrypted string is {}", encrypted.text);

let decrypted: CryptoValue<YourAlgorithm> = cryptor.decrypt(&string);
println!("decrypted string is {}", decrypted.text);
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
