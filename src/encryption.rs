use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit, OsRng},
    AeadCore, Aes256GcmSiv, Nonce,
};
use base64::{engine::general_purpose, Engine};

pub struct Cipher {
    pub(crate) key: Vec<u8>,
    pub(crate) nonce: Vec<u8>,
}

impl Default for Cipher {
    fn default() -> Self {
        let key = Aes256GcmSiv::generate_key(&mut OsRng);
        let nonce = Aes256GcmSiv::generate_nonce(&mut OsRng);

        Cipher {
            key: key.to_vec(),
            nonce: nonce.to_vec(),
        }
    }
}

impl Cipher {
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

    pub fn verify(&self, bytes: &[u8]) -> bool {
        let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&self.key));
        let nonce = Nonce::from_slice(&self.nonce);

        cipher.decrypt(nonce, bytes).is_ok()
    }

    pub fn to_base64(&self) -> (String, String) {
        let key = general_purpose::URL_SAFE_NO_PAD.encode(&self.key);
        let nonce = general_purpose::URL_SAFE_NO_PAD.encode(&self.nonce);

        (key, nonce)
    }

    pub fn from_base64(key: &str, nonce: &str) -> Self {
        let key = general_purpose::URL_SAFE_NO_PAD
            .decode(key.as_bytes())
            .unwrap();
        let nonce = general_purpose::URL_SAFE_NO_PAD
            .decode(nonce.as_bytes())
            .unwrap();

        Cipher { key, nonce }
    }
}
