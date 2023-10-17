use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit, OsRng},
    AeadCore, Aes256GcmSiv, Nonce,
};

pub struct Encryption {
    pub(crate) key: Vec<u8>,
    pub(crate) nonce: Vec<u8>,
}

impl Default for Encryption {
    fn default() -> Self {
        let key = Aes256GcmSiv::generate_key(&mut OsRng);
        let nonce = Aes256GcmSiv::generate_nonce(&mut OsRng);

        Encryption {
            key: key.to_vec(),
            nonce: nonce.to_vec(),
        }
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
