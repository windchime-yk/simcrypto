extern crate aes_gcm_siv;

use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce}; // Or `Aes128GcmSiv`

type SIV = aes_gcm_siv::aead::generic_array::GenericArray<u8, aes_gcm_siv::aead::generic_array::typenum::UInt<aes_gcm_siv::aead::generic_array::typenum::UInt<aes_gcm_siv::aead::generic_array::typenum::UInt<aes_gcm_siv::aead::generic_array::typenum::UInt<aes_gcm_siv::aead::generic_array::typenum::UTerm, aes_gcm_siv::aead::consts::B1>, aes_gcm_siv::aead::consts::B1>, aes_gcm_siv::aead::consts::B0>, aes_gcm_siv::aead::consts::B0>>;

fn generate(keyword: &str, nonces: &'static str) -> (Aes256GcmSiv, &'static SIV) {
    let key = Key::from_slice(keyword.as_bytes());
    let cipher = Aes256GcmSiv::new(key);
    let nonce = Nonce::from_slice(nonces.as_bytes());

    (cipher, nonce)
}

fn encryptor(keyword: &str, nonces: &'static str, data: &str) -> Vec<u8> {
    let generated = generate(keyword, nonces);

    let ciphertext = generated.0
        .encrypt(generated.1, data.as_bytes().as_ref())
        .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

    println!("{:?}", ciphertext);

    ciphertext
}

fn decryptor(keyword: &str, nonces: &'static str, ciphertext: Vec<u8>) -> String {
    let generated = generate(keyword, nonces);

    let plaintext = generated.0
        .decrypt(generated.1, ciphertext.as_ref())
        .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

    String::from_utf8(plaintext.to_vec()).unwrap()
}

fn main() {
    const KEYWORD: &str = "an example very very secret key.";
    const NONCE: &'static str = "unique nonce";

    let ciphertext = encryptor(KEYWORD, NONCE, "plaintext message");
    let plaintext = decryptor(KEYWORD, NONCE, ciphertext);

    println!("{}", plaintext);
    assert_eq!(&plaintext, "plaintext message");
}
