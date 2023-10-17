// encryption module for encrypting and decrypting files using AES-256-GCM-SIV

use aes_gcm_siv::{
    aead::{generic_array::GenericArray, rand_core::RngCore, Aead, KeyInit, OsRng},
    Aes256GcmSiv, Nonce,
};

pub struct Encryption {
    pub(crate) key: Vec<u8>,   // pub(crate) key
    pub(crate) nonce: Vec<u8>, // pub(crate) nonce
}

impl Default for Encryption {
    fn default() -> Self {
        let mut key = vec![0u8; 32];
        let mut nonce = vec![0u8; 12]; 

        OsRng.fill_bytes(&mut key);
        OsRng.fill_bytes(&mut nonce);

        Encryption { key, nonce }
    }
}

impl Encryption {
    pub fn encrypt(&self, bytes: &[u8]) -> Vec<u8> {
        let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&self.key));
        let nonce = Nonce::from_slice(&self.nonce);
        cipher.encrypt(nonce, bytes).unwrap()
    }

    pub fn decrypt(&self, bytes: &[u8]) -> Vec<u8> {
        let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&self.key));
        let nonce = Nonce::from_slice(&self.nonce);
        cipher.decrypt(nonce, bytes).unwrap()
    }
}
