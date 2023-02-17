# Base64 Secret

This is a Rust crate that provides an extension to the Base64 encoding
algorithm. It allows for the alphabet to be shuffled according to a given key,
making it harder to decode data without knowledge of the same key used to encode
it.

This crate is not by any means cryptographically secure, it was designed to be
fast and to be compatible with the slower scripting languages. Do not rely on to
encrypt any sensible data, it was designed to be used as a simple obfuscation
method.

## Installation

To use this crate, add the following line to your Cargo.toml file:

```bash
cargo add base64-secret
```

## Usage

To optimize the usage, and avoid calculating the position of each character in
the alphabet over and over, it is calculated when the struct is created.

```rust
use base64_secret::Base64;

let engine = Base64::new(b"my secret key");
let data = b"This is a secret message";

let encoded_data = engine.encode(&data);

println!("{}", encoded_data); // prints "v-O0BPA0BPAOhl1yZm9yJQAuRz1XZ7Jy"
```

To decode is quite simple as well:

```rust
use base64_secret::Base64;

let engine = Base64::new(b"my secret key");

let encoded_data = "jPF_7wA_7wAFLXlCbaHCzQAS3flubnzC";
let data = engine.decode(&encoded_data).unwrap();

println!("{}", String::from_utf8_lossy(&data)); // prints "This is a secret message"
```

## Contributing

Contributions are welcome! If you find a bug or have a feature request, please
open an issue on the GitHub repository. If you'd like to contribute code, please
open a pull request.

## License

This crate is licensed under the MIT license. See the LICENSE file for more
information.
