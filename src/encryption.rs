use aes_gcm_siv::{
    aead::{
        consts::{U12, U32},
        generic_array::GenericArray,
        Aead, KeyInit, OsRng,
    },
    AeadCore, Aes256GcmSiv,
};
use base64::{engine::general_purpose, Engine};

pub struct Cipher {
    cipher: Aes256GcmSiv,
    key: GenericArray<u8, U32>,
    nonce: GenericArray<u8, U12>,
}

impl Default for Cipher {
    fn default() -> Self {
        let key = Aes256GcmSiv::generate_key(&mut OsRng);
        let nonce = Aes256GcmSiv::generate_nonce(&mut OsRng);
        let cipher = Aes256GcmSiv::new(GenericArray::from_slice(&key));

        Cipher { cipher, key, nonce }
    }
}

impl Cipher {
    #[must_use]
    pub fn encrypt(&self, bytes: &[u8]) -> Result<Vec<u8>, aes_gcm_siv::aead::Error> {
        self.cipher.encrypt(&self.nonce, bytes)
    }

    #[must_use]
    pub fn decrypt(&self, bytes: &[u8]) -> Result<Vec<u8>, aes_gcm_siv::aead::Error> {
        self.cipher.decrypt(&self.nonce, bytes)
    }

    #[must_use]
    pub fn verify(&self, bytes: &[u8]) -> bool {
        self.cipher.decrypt(&self.nonce, bytes).is_ok()
    }

    #[must_use]
    pub fn to_base64(&self) -> (String, String) {
        let key = general_purpose::URL_SAFE_NO_PAD.encode(self.key);
        let nonce = general_purpose::URL_SAFE_NO_PAD.encode(self.nonce);

        (key, nonce)
    }

    #[must_use]
    pub fn from_base64(key: &str, nonce: &str) -> Result<Self, base64::DecodeError> {
        let key = general_purpose::URL_SAFE_NO_PAD.decode(key.as_bytes())?;
        let nonce = general_purpose::URL_SAFE_NO_PAD.decode(nonce.as_bytes())?;

        Ok(Cipher {
            cipher: Aes256GcmSiv::new(GenericArray::from_slice(&key)),
            key: GenericArray::clone_from_slice(&key),
            nonce: GenericArray::clone_from_slice(&nonce),
        })
    }
}
