use std::{
    ffi::c_void,
    io::{Read, Write},
};

use libsodium_sys::{
    crypto_box_detached, crypto_kx_PUBLICKEYBYTES, crypto_kx_SECRETKEYBYTES,
    crypto_kx_SESSIONKEYBYTES, crypto_kx_keypair, crypto_kx_server_session_keys,
    crypto_secretbox_detached, crypto_secretbox_open_detached,
    crypto_secretbox_xsalsa20poly1305_MACBYTES, crypto_secretbox_xsalsa20poly1305_NONCEBYTES,
    randombytes_buf, sodium_increment,
};
use thiserror::Error;

use super::ClientProtocolConfig;

pub const PUBLICKEYBYTES: usize = crypto_kx_PUBLICKEYBYTES as usize;
pub const SECRETKEYBYTES: usize = crypto_kx_SECRETKEYBYTES as usize;
pub const SESSIONKEYBYTES: usize = crypto_kx_SESSIONKEYBYTES as usize;
pub const NONCEBYTES: usize = crypto_secretbox_xsalsa20poly1305_NONCEBYTES as usize;
pub const MACBYTES: usize = crypto_secretbox_xsalsa20poly1305_MACBYTES as usize;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Io {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not generate server keys.")]
    ServerKeyGeneration,
    #[error("Could not encrypt buffer with kx encryption parameters.")]
    KxEncrypt,
    #[error("Could not decrypt buffer with kx encryption parameters.")]
    KxDecrypt,
    #[error("Client sent invalid public key.")]
    InvalidClientPublicKey,
    #[error("Could not generate session keys.")]
    SessionKeyGeneration,
    #[error("The kx is not complete.")]
    SessionNotInitialized,
    #[error("Could not decrypt session message with session parameters.")]
    SessionDecrypt,
    #[error("Could not encrypt session message with session parameters.")]
    SessionEncrypt,
}

pub struct ClientProtocolCrypto {
    kx_client_public_key: Vec<u8>,
    kx_server_secret_key: Vec<u8>,
    server_public_key: [u8; PUBLICKEYBYTES],
    server_secret_key: [u8; SECRETKEYBYTES],
    server_nonce: [u8; NONCEBYTES],
    client_nonce: [u8; NONCEBYTES],
    session_keys: Option<([u8; SESSIONKEYBYTES], [u8; SESSIONKEYBYTES])>,
}

impl ClientProtocolCrypto {
    /// Creates a new instance and generates session keys
    pub fn generate(config: &ClientProtocolConfig) -> Result<ClientProtocolCrypto, Error> {
        let mut server_public_key = [0u8; PUBLICKEYBYTES];
        let mut server_secret_key = [0u8; SECRETKEYBYTES];

        // Generate server-ended keypair.
        if unsafe {
            crypto_kx_keypair(
                server_public_key.as_mut_ptr(),
                server_secret_key.as_mut_ptr(),
            )
        } != 0
        {
            return Err(Error::ServerKeyGeneration);
        }

        // Generate both server and client nonces.
        let mut server_nonce = [0u8; NONCEBYTES];
        let mut client_nonce = [0u8; NONCEBYTES];
        unsafe {
            randombytes_buf(server_nonce.as_mut_ptr() as *mut c_void, NONCEBYTES);
            randombytes_buf(client_nonce.as_mut_ptr() as *mut c_void, NONCEBYTES);
        };

        Ok(Self {
            kx_server_secret_key: config.kx_server_secret_key.clone(),
            kx_client_public_key: config.kx_client_public_key.clone(),
            server_public_key,
            server_secret_key,
            server_nonce,
            client_nonce,
            session_keys: None,
        })
    }

    /// Produces an advertisement buffer for sending to the client.
    /// Handles the encryption of the output buffer internally.
    pub fn create_session_advertisement(&self) -> Result<Vec<u8>, Error> {
        // Build plaintext crypto session parameters message.
        let mut plain = vec![];
        plain.write_all(&self.server_public_key)?;
        plain.write_all(&self.server_nonce)?;
        plain.write_all(&self.client_nonce)?;

        // Generate a random one-off nonce for the kx message.
        let mut bootstrap_nonce = [0u8; NONCEBYTES];
        unsafe {
            randombytes_buf(bootstrap_nonce.as_mut_ptr() as *mut c_void, NONCEBYTES);
        };

        // Encrypt plaintext using preshared keys and generated parameters.
        let mut mac = [0u8; MACBYTES];
        if unsafe {
            crypto_box_detached(
                plain.as_mut_ptr(),
                mac.as_mut_ptr(),
                plain.as_ptr(),
                plain.len() as u64,
                bootstrap_nonce.as_ptr(),
                self.kx_client_public_key.as_ptr(),
                self.kx_server_secret_key.as_ptr(),
            )
        } != 0
        {
            return Err(Error::KxEncrypt);
        }

        // Frame the outgoing kx message with the generated nonce and mac.
        let mut framed = vec![];
        framed.write_all(&bootstrap_nonce)?;
        framed.write_all(&mac)?;
        framed.write_all(&plain)?;
        Ok(framed)
    }

    /// Prepare the state for decrypting in a session by supplying appropriate client keys.
    /// Handles the decryption of the keys internally.
    pub fn post_client_keys<R>(&mut self, mut data: R) -> Result<(), Error>
    where
        R: Read,
    {
        // Split received data into its components
        let mut nonce = vec![0; NONCEBYTES];
        let mut mac = vec![0; MACBYTES];
        let mut client_public_key = vec![];
        data.read_exact(nonce.as_mut_slice())?;
        data.read_exact(mac.as_mut_slice())?;
        data.read_to_end(&mut client_public_key)?;

        if client_public_key.len() != PUBLICKEYBYTES {
            return Err(Error::InvalidClientPublicKey);
        }

        // Decrypt the client pk with the read parameters and the preshared keys.
        if unsafe {
            libsodium_sys::crypto_box_open_detached(
                client_public_key.as_mut_ptr(),
                client_public_key.as_ptr(),
                mac.as_ptr(),
                client_public_key.len() as u64,
                nonce.as_ptr(),
                self.kx_client_public_key.as_ptr(),
                self.kx_server_secret_key.as_ptr(),
            )
        } != 0
        {
            return Err(Error::KxDecrypt);
        }

        // Derive the keys to-be used for this crypto session.
        let mut rx = [0u8; SESSIONKEYBYTES];
        let mut tx = [0u8; SESSIONKEYBYTES];
        if unsafe {
            crypto_kx_server_session_keys(
                rx.as_mut_ptr(),
                tx.as_mut_ptr(),
                self.server_public_key.as_ptr(),
                self.server_secret_key.as_ptr(),
                client_public_key.as_ptr(),
            )
        } != 0
        {
            return Err(Error::SessionKeyGeneration);
        }

        self.session_keys = Some((rx, tx));

        Ok(())
    }

    /// Decrypt a message from the current session.
    pub fn decrypt<R>(&mut self, mut data: R) -> Result<Vec<u8>, Error>
    where
        R: Read,
    {
        let session_keys = self
            .session_keys
            .as_ref()
            .ok_or(Error::SessionNotInitialized)?;

        // Split out received buffer
        let mut mac = vec![0; MACBYTES];
        let mut contents = vec![];
        data.read_exact(mac.as_mut_slice())?;
        data.read_to_end(&mut contents)?;

        // Decrypt contents in-place
        if unsafe {
            crypto_secretbox_open_detached(
                contents.as_mut_ptr(),
                contents.as_ptr(),
                mac.as_ptr(),
                contents.len() as u64,
                self.client_nonce.as_ptr(),
                session_keys.0.as_ptr(),
            )
        } != 0
        {
            return Err(Error::SessionDecrypt);
        }

        unsafe { sodium_increment(self.client_nonce.as_mut_ptr(), self.client_nonce.len()) }

        Ok(contents)
    }

    /// Encrypt a message for the current session.
    pub fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let mut data = data.to_vec();
        let session_keys = self
            .session_keys
            .as_ref()
            .ok_or(Error::SessionNotInitialized)?;

        // Encrypt the data.
        let mut mac = [0u8; MACBYTES];
        if unsafe {
            crypto_secretbox_detached(
                data.as_mut_ptr(),
                mac.as_mut_ptr(),
                data.as_ptr(),
                data.len() as u64,
                self.server_nonce.as_ptr(),
                session_keys.1.as_ptr(),
            )
        } != 0
        {
            return Err(Error::SessionEncrypt);
        }

        // Prepend the mac onto the encrypted data.
        let mut framed = vec![];
        framed.write_all(&mac)?;
        framed.write_all(&data)?;

        unsafe { sodium_increment(self.server_nonce.as_mut_ptr(), self.server_nonce.len()) }

        Ok(framed)
    }
}
