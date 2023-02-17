//! # Base64 with custom alphabet
//!
//! Base64 encoder/decoder with custom alphabet. The alphabet is sorted by a
//! given key. The sorting is always deterministic.
//!
//! The idea is to make it virtually impossible to decode the encoded data
//! without the key. The harder to guess the key, the harder to decode/decode
//! data.
//!
//! This crate is not by any means cryptographically secure, it was designed to
//! be fast and to be compatible with the slower scripting languages. Do not
//! rely on to encrypt any sensible data, it was designed to be used as a simple
//! obfuscation method.
#![deny(missing_docs)]
#![allow(warnings)]

pub use base64::DecodeError as Error;
use base64::{alphabet, engine, Engine};

/// Base64 encoder/decoder with custom alphabet.
///
/// The alphabet is sorted by a given key and their weights are calculated by
/// the CRC32 hash of each character.
///
/// This class will provide a custom alphabet for the base64 encoder/decoder,
/// which makes virtually impossible to decode the encoded data without the key.
pub struct Base64 {
    /// Base64 engine.
    engine: engine::GeneralPurpose,
}

impl Base64 {
    /// Create a new Base64 encoder/decoder with a custom key to sort the alphabet.
    pub fn new(key: &[u8]) -> Self {
        let alphabet = alphabet::Alphabet::new(&Self::get_alphabet(key)).expect("alphabet");
        let config = engine::GeneralPurposeConfig::new()
            .with_decode_allow_trailing_bits(false)
            .with_encode_padding(false)
            .with_decode_padding_mode(engine::DecodePaddingMode::RequireNone);

        Self {
            engine: engine::GeneralPurpose::new(&alphabet, config),
        }
    }

    /// Get a custom alphabet sorted by the given key.
    fn get_alphabet(key: &[u8]) -> String {
        let rev_key = key.iter().cloned().rev().collect::<Vec<_>>();
        let hash = crc32fast::hash(key);
        let rev_hash = crc32fast::hash(&rev_key);

        let mut alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_"
            .chars()
            .enumerate()
            .map(|(i, c)| {
                (
                    c,
                    crc32fast::hash(c.to_string().as_bytes())
                        % if i % 2 == 0 { hash } else { rev_hash },
                )
            })
            .collect::<Vec<(char, u32)>>();

        alphabet.sort_by(|a, b| b.1.cmp(&a.1));

        alphabet.iter().map(|a| a.0).collect::<String>()
    }

    /// Encode the given input with the custom alphabet.
    pub fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
        self.engine.encode(input)
    }

    /// Decode the given input with the custom alphabet.
    pub fn decode<T: AsRef<[u8]>>(&self, input: T) -> Result<Vec<u8>, Error> {
        self.engine.decode(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_decode() {
        let x = Base64::new(b"test");
        let encoded = x.encode("test");
        let decoded = x.decode(encoded.as_bytes()).expect("success");
        assert_eq!("test".to_owned(), String::from_utf8_lossy(&decoded));
    }

    #[test]
    fn incorrect_key() {
        let x = Base64::new(b"test");
        let y = Base64::new(b"test1");
        let encoded = x.encode("test");
        assert!(y.decode(encoded.as_bytes()).is_err());
        assert!(x.decode(encoded.as_bytes()).is_ok());
    }

    #[test]
    fn expected_data() {
        let x = Base64::new(b"long and random key\0test\0");
        let result = x.decode("t0mvt-").expect("decode");
        assert_eq!("test".to_owned(), String::from_utf8_lossy(&result));
    }
}
