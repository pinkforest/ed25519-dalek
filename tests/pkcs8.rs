//! PKCS#8 private key and SPKI public key tests.
//!
//! These are standard formats for storing public and private keys, defined in
//! RFC5958 (PKCS#8) and RFC5280 (SPKI).

#![cfg(feature = "pkcs8")]

use ed25519_dalek::pkcs8::{DecodePrivateKey, DecodePublicKey};
use ed25519_dalek::{Keypair, PublicKey};
use hex_literal::hex;

#[cfg(feature = "alloc")]
use ed25519_dalek::{
    pkcs8::{EncodePrivateKey, EncodePublicKey},
    SecretKey,
};

/// Ed25519 PKCS#8 v1 private key encoded as ASN.1 DER.
const PKCS8_V1_DER: &[u8] = include_bytes!("examples/pkcs8-v1.der");

/// Ed25519 PKCS#8 v2 private key + public key encoded as ASN.1 DER.
const PKCS8_V2_DER: &[u8] = include_bytes!("examples/pkcs8-v2.der");

/// Ed25519 SubjectPublicKeyInfo encoded as ASN.1 DER.
const PUBLIC_KEY_DER: &[u8] = include_bytes!("examples/pubkey.der");

/// Secret key bytes.
///
/// Extracted with:
/// $ openssl asn1parse -inform der -in tests/examples/pkcs8-v1.der
const SK_BYTES: [u8; 32] = hex!("D4EE72DBF913584AD5B6D8F1F769F8AD3AFE7C28CBF1D4FBE097A88F44755842");

/// Public key bytes.
const PK_BYTES: [u8; 32] = hex!("19BF44096984CDFE8541BAC167DC3B96C85086AA30B6B6CB0C5C38AD703166E1");

#[test]
fn decode_pkcs8_v1() {
    let keypair = Keypair::from_pkcs8_der(PKCS8_V1_DER).unwrap();
    assert_eq!(SK_BYTES, keypair.secret_key().to_bytes());
    assert_eq!(PK_BYTES, keypair.public_key().to_bytes());
}

#[test]
fn decode_pkcs8_v2() {
    let keypair = Keypair::from_pkcs8_der(PKCS8_V2_DER).unwrap();
    assert_eq!(SK_BYTES, keypair.secret_key().to_bytes());
    assert_eq!(PK_BYTES, keypair.public_key().to_bytes());
}

#[test]
fn decode_public_key() {
    let public_key = PublicKey::from_public_key_der(PUBLIC_KEY_DER).unwrap();
    assert_eq!(PK_BYTES, public_key.to_bytes());
}

#[test]
#[cfg(feature = "alloc")]
fn encode_pkcs8() {
    let keypair = Keypair::from(SecretKey::from_bytes(&SK_BYTES).unwrap());
    let pkcs8_key = keypair.to_pkcs8_der().unwrap();

    let keypair2 = Keypair::from_pkcs8_der(pkcs8_key.as_bytes()).unwrap();
    assert_eq!(keypair.to_bytes(), keypair2.to_bytes());
}

#[test]
#[cfg(feature = "alloc")]
fn encode_public_key() {
    let public_key = PublicKey::from_bytes(&PK_BYTES).unwrap();
    let public_key_der = public_key.to_public_key_der().unwrap();

    let public_key2 = PublicKey::from_public_key_der(public_key_der.as_bytes()).unwrap();
    assert_eq!(public_key, public_key2);
}
