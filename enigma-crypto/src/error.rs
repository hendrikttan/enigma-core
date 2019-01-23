use failure::Fail;
use crate::std::string::String;

#[derive(Debug, Fail)]
pub enum CryptoError {
    #[fail(display = "Failed to derive a key with ECDH: self: {}, other: {}", self_key, other_key)]
    DerivingKeyError { self_key: String, other_key: String },

    #[fail(display = "The {} Isn't valid", key_type)]
    KeyError { key_type: String },

    #[fail(display = "Failed Decrypting")]
    DecryptionError,

    #[fail(display = "Improper Encryption")]
    ImproperEncryption,

    #[fail(display = "Failed Encrypting")]
    EncryptionError,

    #[fail(display = "Signing the message failed: {}", msg)]
    SigningError { msg: String },

    #[fail(display = "Failed Generating a: {}", err)]
    RandomError { err: String },
}