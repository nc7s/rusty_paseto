#![cfg(feature = "v1_local")]
use std::marker::PhantomData;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{KeyIvInit, StreamCipher};
use crate::core::common::cipher_text::CipherText;
use crate::core::{Local, V1};
use crate::core::common::EncryptionKey;

type Aes256Ctr = ctr::Ctr64BE<aes::Aes256>;

impl CipherText<V1, Local> {
    pub(crate) fn from(payload: &[u8], encryption_key: &EncryptionKey<V1, Local>) -> Self {
        let key = GenericArray::from_slice(encryption_key.as_ref());
        let nonce = GenericArray::from_slice(encryption_key.counter_nonce());
        let mut cipher = Aes256Ctr::new(key, nonce);
        let mut ciphertext = vec![0u8; payload.as_ref().len()];

        ciphertext.copy_from_slice(payload);

        cipher.apply_keystream(&mut ciphertext);

        CipherText {
            ciphertext,
            version: PhantomData,
            purpose: PhantomData,
        }
    }
}
