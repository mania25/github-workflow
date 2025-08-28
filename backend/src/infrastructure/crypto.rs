use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use ring::{
    aead,
    rand::{SecureRandom, SystemRandom},
};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct PQCrypto {
    server_key: aead::LessSafeKey,
    rng: SystemRandom,
    session_keys: Mutex<HashMap<Vec<u8>, aead::LessSafeKey>>,
}

impl PQCrypto {
    pub fn new() -> Result<Self> {
        let rng = SystemRandom::new();
        let server_key_bytes = {
            let mut key = [0u8; 32];
            rng.fill(&mut key)
                .map_err(|_| anyhow!("Failed to generate server key"))?;
            key
        };

        let unbound_key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &server_key_bytes)
            .map_err(|_| anyhow!("Failed to create server key"))?;
        let server_key = aead::LessSafeKey::new(unbound_key);

        Ok(Self {
            server_key,
            rng,
            session_keys: Mutex::new(HashMap::new()),
        })
    }

    pub fn generate_session_key(&self, client_public_key: &[u8]) -> Result<[u8; 32]> {
        let mut session_key_bytes = [0u8; 32];
        self.rng
            .fill(&mut session_key_bytes)
            .map_err(|_| anyhow!("Failed to generate session key"))?;

        let unbound_key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &session_key_bytes)
            .map_err(|_| anyhow!("Failed to create session key"))?;
        let session_key = aead::LessSafeKey::new(unbound_key);

        let mut session_keys = self
            .session_keys
            .lock()
            .map_err(|_| anyhow!("Failed to acquire session keys lock"))?;
        session_keys.insert(client_public_key.to_vec(), session_key);

        Ok(session_key_bytes)
    }

    pub fn encrypt(&self, data: &str) -> Result<String> {
        let mut nonce_bytes = [0u8; 12];
        self.rng
            .fill(&mut nonce_bytes)
            .map_err(|_| anyhow!("Failed to generate nonce"))?;

        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);
        let mut in_out = data.as_bytes().to_vec();

        self.server_key
            .seal_in_place_append_tag(nonce, aead::Aad::from(b"todo-app"), &mut in_out)
            .map_err(|_| anyhow!("Failed to encrypt data"))?;

        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&in_out);

        Ok(general_purpose::STANDARD.encode(&result))
    }

    pub fn decrypt(&self, encrypted_data: &str) -> Result<String> {
        let data = general_purpose::STANDARD
            .decode(encrypted_data)
            .map_err(|_| anyhow!("Failed to decode base64"))?;

        if data.len() < 12 {
            return Err(anyhow!("Invalid encrypted data length"));
        }

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = aead::Nonce::try_assume_unique_for_key(nonce_bytes)
            .map_err(|_| anyhow!("Invalid nonce"))?;

        let mut in_out = ciphertext.to_vec();
        let plaintext = self
            .server_key
            .open_in_place(nonce, aead::Aad::from(b"todo-app"), &mut in_out)
            .map_err(|_| anyhow!("Failed to decrypt data"))?;

        String::from_utf8(plaintext.to_vec())
            .map_err(|_| anyhow!("Invalid UTF-8 in decrypted data"))
    }

    pub fn decrypt_with_session(
        &self,
        encrypted_data: &str,
        client_public_key: &[u8],
    ) -> Result<String> {
        let session_keys = self
            .session_keys
            .lock()
            .map_err(|_| anyhow!("Failed to acquire session keys lock"))?;

        let session_key = session_keys
            .get(client_public_key)
            .ok_or_else(|| anyhow!("Session key not found"))?;

        let data = general_purpose::STANDARD
            .decode(encrypted_data)
            .map_err(|_| anyhow!("Failed to decode base64"))?;

        if data.len() < 12 {
            return Err(anyhow!("Invalid encrypted data length"));
        }

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let nonce = aead::Nonce::try_assume_unique_for_key(nonce_bytes)
            .map_err(|_| anyhow!("Invalid nonce"))?;

        let mut in_out = ciphertext.to_vec();
        let plaintext = session_key
            .open_in_place(nonce, aead::Aad::from(b"todo-app"), &mut in_out)
            .map_err(|_| anyhow!("Failed to decrypt data"))?;

        String::from_utf8(plaintext.to_vec())
            .map_err(|_| anyhow!("Invalid UTF-8 in decrypted data"))
    }
}
