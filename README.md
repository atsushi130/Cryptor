# Encrypter
Encrypter is encryption machine corresponding to the diversity of algorithms.

## Dependencies
Insert to Cargo.toml of your project.
```toml
[dependencies]
encrypter = "1.0.0"
```
or
```console
// Newest version
$ cargo add encrypter

// Version specification
$ cargo add encrypter@1.0.0

// If not exist on crates.io
$ mkdir lib
$ cd lib
$ git clone https://github.com/atsushi130/Encrypter
$ cd ..
$ cargo add encrypter --path=lib/encrypter/
```

## Usage
Implement structure with this Algorithm trait.
```rust
pub trait Algorithm {
    type V: Algorithm;
    fn encrypt(&mut self, character: &char) -> EncryptValue<Self::V>;
}
```

Encrypter have member with Algorithm trait. Dependency injection your implemented structure to Encrypter.
```rust
let mut encrypter = Encrypter {
    hash: YourAlgorithm { ... }
};
```

Return type of encrypt method is `EncryptValue<YourAlgorithm>`;
```rust
let encrypted: EncryptValue<YourAlgorithm> = encrypter.encrypt(&character);
println!("encrypted character is {}", encrypted.text);
```

## Run
```console
$ cargo build
$ cargo run
```

## Test
```console
$ cargo test
```
