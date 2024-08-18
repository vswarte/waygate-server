use std::ffi::c_void;
use std::io::{self, Read};
use libsodium_sys::{
    crypto_kx_PUBLICKEYBYTES,
    crypto_kx_SECRETKEYBYTES,
    crypto_kx_SESSIONKEYBYTES,
    crypto_secretbox_xsalsa20poly1305_MACBYTES,
    crypto_secretbox_xsalsa20poly1305_NONCEBYTES,
    crypto_box_detached,
    crypto_kx_keypair,
    crypto_kx_server_session_keys,
    crypto_secretbox_detached,
    crypto_secretbox_open_detached,
    randombytes_buf,
    sodium_increment,
};

use thiserror::Error;
use tokio::io::AsyncWriteExt;

use crate::{ClientError, KX_CLIENT_PUBLIC_KEY, KX_SERVER_SECRET_KEY};

pub const PUBLICKEYBYTES: usize = crypto_kx_PUBLICKEYBYTES as usize;
pub const SECRETKEYBYTES: usize = crypto_kx_SECRETKEYBYTES as usize;
pub const SESSIONKEYBYTES: usize = crypto_kx_SESSIONKEYBYTES as usize;
pub const NONCEBYTES: usize = crypto_secretbox_xsalsa20poly1305_NONCEBYTES as usize;
pub const MACBYTES: usize = crypto_secretbox_xsalsa20poly1305_MACBYTES as usize;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Could not derive session key.")]
    SessionKeyDerivation,

    #[error("Could not decrypt message.")]
    Decrypt,

    #[error("Could not encrypt message.")]
    Encrypt,

    #[error("Key exchange failed.")]
    KeyExchange,
}

pub fn new_client_crypto() -> ClientCrypto<ClientCryptoStateCreated> {
    ClientCrypto { state: ClientCryptoStateCreated {  } }
}

pub trait ClientCryptoState {}

#[derive(Debug)]
pub struct ClientCrypto<S: ClientCryptoState> {
    state: S,
}

#[derive(Debug)]
pub struct ClientCryptoStateCreated {}

impl ClientCryptoState for ClientCryptoStateCreated {}

impl ClientCrypto<ClientCryptoStateCreated> {
    pub fn generate_new(self) -> ClientCrypto<ClientCryptoStateParametersGenerated> {
        let mut server_pk = [0u8; PUBLICKEYBYTES];
        let mut server_sk = [0u8; SECRETKEYBYTES];

        let result = unsafe {
            crypto_kx_keypair(server_pk.as_mut_ptr(), server_sk.as_mut_ptr())
        };
        if result != 0 {
            unimplemented!();
        }

        let mut server_nonce = [0u8; NONCEBYTES];
        let mut client_nonce = [0u8; NONCEBYTES];
        unsafe {
            randombytes_buf(server_nonce.as_mut_ptr() as *mut c_void, NONCEBYTES);
            randombytes_buf(client_nonce.as_mut_ptr() as *mut c_void, NONCEBYTES);
        };

        ClientCrypto {
            state: ClientCryptoStateParametersGenerated {
                server_pk,
                server_sk,
                server_nonce,
                client_nonce,
            }
        }
    }
}

#[derive(Debug)]
pub struct ClientCryptoStateParametersGenerated {
    server_pk: [u8; PUBLICKEYBYTES],
    server_sk: [u8; SECRETKEYBYTES],
    server_nonce: [u8; NONCEBYTES],
    client_nonce: [u8; NONCEBYTES],
}

impl ClientCryptoState for ClientCryptoStateParametersGenerated {}

impl ClientCrypto<ClientCryptoStateParametersGenerated> {
    pub async fn create_crypto_advertisement_buffer(
        &self
    ) -> Result<Vec<u8>, ClientError> {
        let mut buffer = vec![];

        buffer.write_all(&self.state.server_pk)
            .await.map_err(ClientError::Io)?;
        buffer.write_all(&self.state.server_nonce)
            .await.map_err(ClientError::Io)?;
        buffer.write_all(&self.state.client_nonce)
            .await.map_err(ClientError::Io)?;

        Ok(buffer)
    }

    pub fn derive_session_keys(
        self,
        client_pk: [u8; PUBLICKEYBYTES],
    ) -> Result<ClientCrypto<ClientCryptoStateActiveSession>, ClientError>{
        let mut rx = [0u8; SESSIONKEYBYTES];
        let mut tx = [0u8; SESSIONKEYBYTES];

        let result = unsafe {
            crypto_kx_server_session_keys(
                rx.as_mut_ptr(),
                tx.as_mut_ptr(),
                self.state.server_pk.as_ptr(),
                self.state.server_sk.as_ptr(),
                client_pk.as_ptr(),
            )
        };

        if result != 0 {
            return Err(ClientError::Crypto(CryptoError::SessionKeyDerivation))
        }

        Ok(ClientCrypto {
            state: ClientCryptoStateActiveSession {
                server_nonce: self.state.server_nonce,
                client_nonce: self.state.client_nonce,
                rx,
                tx,
            }
        })
    }

    pub async fn kx_decrypt<R: Read>(&mut self, r: R) -> Result<Vec<u8>, ClientError> {
        let (nonce, mac, mut payload) = Self::split_nonce_mac_and_payload(r)
            .await.map_err(ClientError::Io)?;

        let result = unsafe {
            libsodium_sys::crypto_box_open_detached(
                payload.as_mut_ptr(),
                payload.as_ptr(),
                mac.as_ptr(),
                payload.len() as u64,
                nonce.as_ptr(),
                KX_CLIENT_PUBLIC_KEY.get().unwrap().as_ptr(),
                KX_SERVER_SECRET_KEY.get().unwrap().as_ptr(),
            )
        };

        if result != 0 {
            return Err(ClientError::Crypto(CryptoError::Decrypt));
        }

        Ok(payload)
    }

    async fn split_nonce_mac_and_payload<R: Read>(mut r: R) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), io::Error> {
        let mut nonce = vec![0; NONCEBYTES];
        let mut mac = vec![0; MACBYTES];
        let mut payload = vec![];

        r.read_exact(nonce.as_mut_slice())?;
        r.read_exact(mac.as_mut_slice())?;
        r.read_to_end(&mut payload)?;

        Ok((nonce, mac, payload))
    }

    pub async fn kx_encrypt(
        &self,
        mut payload: Vec<u8>
    ) -> Result<Vec<u8>, ClientError> {
        let mut bootstrap_nonce = [0u8; NONCEBYTES];
        unsafe {
            randombytes_buf(bootstrap_nonce.as_mut_ptr() as *mut c_void, NONCEBYTES);
        };

        let mut mac = [0u8; MACBYTES];
        let result = unsafe {
            crypto_box_detached(
                payload.as_mut_ptr(),
                mac.as_mut_ptr(),
                payload.as_ptr(),
                payload.len() as u64,
                bootstrap_nonce.as_ptr(),
                KX_CLIENT_PUBLIC_KEY.get().unwrap().as_ptr(),
                KX_SERVER_SECRET_KEY.get().unwrap().as_ptr(),
            )
        };

        if result != 0 {
            return Err(ClientError::Crypto(CryptoError::KeyExchange))
        }

        let message = Self::frame_kx_message(
            &bootstrap_nonce,
            mac,
            payload.as_slice(),
        ).await?;

        Ok(message)
    }

    async fn frame_kx_message(
        nonce: &[u8; NONCEBYTES],
        mac: [u8; MACBYTES],
        payload: &[u8],
    ) -> Result<Vec<u8>, io::Error> {
        let mut buffer = vec![];

        buffer.write_all(nonce).await?;
        buffer.write_all(&mac).await?;
        buffer.write_all(payload).await?;

        Ok(buffer)
    }
}

#[derive(Debug)]
pub struct ClientCryptoStateActiveSession {
    server_nonce: [u8; NONCEBYTES],
    client_nonce: [u8; NONCEBYTES],
    rx: [u8; SESSIONKEYBYTES],
    tx: [u8; SESSIONKEYBYTES],
}

impl ClientCryptoState for ClientCryptoStateActiveSession {}

impl ClientCrypto<ClientCryptoStateActiveSession> {
    pub async fn session_decrypt<R: Read>(&mut self, r: R) -> Result<Vec<u8>, ClientError> {
        let (mac, mut payload) = Self::split_mac_and_payload(r).await?;

        let result = unsafe {
            crypto_secretbox_open_detached(
                payload.as_mut_ptr(),
                payload.as_ptr(),
                mac.as_ptr(),
                payload.len() as u64,
                self.state.client_nonce.as_ptr(),
                self.state.rx.as_ptr(),
            )
        };

        if result != 0 {
            return Err(ClientError::Crypto(CryptoError::Decrypt))
        }

        unsafe {
            sodium_increment(
                self.state.client_nonce.as_mut_ptr(),
                self.state.client_nonce.len(),
            )
        }

        Ok(payload)
    }

    pub async fn session_encrypt(
        &mut self,
        payload: &mut Vec<u8>
    ) -> Result<Vec<u8>, ClientError> {
        let mut mac = [0u8; MACBYTES];
        let result = unsafe {
            crypto_secretbox_detached(
                payload.as_mut_ptr(),
                mac.as_mut_ptr(),
                payload.as_ptr(),
                payload.len() as u64,
                self.state.server_nonce.as_ptr(),
                self.state.tx.as_ptr(),
            )
        };

        if result != 0 {
            return Err(ClientError::Crypto(CryptoError::Encrypt))
        }

        let message = Self::frame_payload(mac, payload).await?;

        unsafe {
            sodium_increment(
                self.state.server_nonce.as_mut_ptr(),
                self.state.server_nonce.len(),
            )
        }

        Ok(message)
    }

    async fn frame_payload(
        mac: [u8; MACBYTES],
        payload: &[u8],
    ) -> Result<Vec<u8>, io::Error> {
        let mut buffer = vec![];

        buffer.write_all(&mac).await?;
        buffer.write_all(payload).await?;

        Ok(buffer)
    }

    async fn split_mac_and_payload<R: Read>(mut r: R) -> Result<(Vec<u8>, Vec<u8>), io::Error> {
        let mut mac = vec![0; MACBYTES];
        let mut payload = vec![];

        r.read_exact(mac.as_mut_slice())?;
        r.read_to_end(&mut payload)?;

        Ok((mac, payload))
    }
}
