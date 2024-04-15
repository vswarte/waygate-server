use std::io;
use sodiumoxide::crypto;
use thiserror::Error;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

use crate::client::ClientError;

use super::key;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Could not derive session key")]
    SessionKeyDerivationFailed,

    #[error("Could not create a public key for session")]
    PublicKeyCreationFailed,

    #[error("Could not create a secret key for session")]
    SecretKeyCreationFailed,

    #[error("Could not create a MAC for message")]
    MacCreationFailed,

    #[error("Could not create a none for message")]
    NonceCreationFailed,

    #[error("Could not decrypt message")]
    DecryptFailed,

    #[error("Could not convert keys")]
    KeyConversionFailed,
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
        let (server_pk, server_sk) = crypto::kx::x25519blake2b::gen_keypair();
        let server_nonce = crypto::secretbox::gen_nonce();
        let client_nonce = crypto::secretbox::gen_nonce();

        ClientCrypto { state: ClientCryptoStateParametersGenerated {
            server_pk,
            server_sk,
            server_nonce,
            client_nonce,
        } }
    }
}

#[derive(Debug)]
pub struct ClientCryptoStateParametersGenerated {
    server_pk: crypto::kx::x25519blake2b::PublicKey,
    server_sk: crypto::kx::x25519blake2b::SecretKey,
    server_nonce: crypto::secretbox::Nonce,
    client_nonce: crypto::secretbox::Nonce,
}

impl ClientCryptoState for ClientCryptoStateParametersGenerated {}

impl ClientCrypto<ClientCryptoStateParametersGenerated> {
    pub async fn create_crypto_advertisement_buffer(
        &self
    ) -> Result<Vec<u8>, ClientError> {
        let mut writer = io::Cursor::new(vec![]);

        writer.write_all(&self.state.server_pk.0)
            .await.map_err(ClientError::Io)?;
        writer.write_all(&self.state.server_nonce.0)
            .await.map_err(ClientError::Io)?;
        writer.write_all(&self.state.client_nonce.0)
            .await.map_err(ClientError::Io)?;

        Ok(writer.into_inner())
    }

    pub fn derive_session_keys(
        self,
        client_pk: crypto::kx::x25519blake2b::PublicKey,
    ) -> Result<ClientCrypto<ClientCryptoStateActiveSession>, ClientError>{
        let (rx, tx) = crypto::kx::x25519blake2b::server_session_keys(
            &self.state.server_pk,
            &self.state.server_sk,
            &client_pk,
        ).map_err(|_| ClientError::Crypto(CryptoError::SessionKeyDerivationFailed))?;

        Ok(ClientCrypto {
            state: ClientCryptoStateActiveSession {
                server_nonce: self.state.server_nonce,
                client_nonce: self.state.client_nonce,
                rx,
                tx,
            }
        })
    }

    pub async fn kx_decrypt(
        &mut self,
        message: &[u8]
    ) -> Result<Vec<u8>, ClientError> {
        let (nonce, mac, mut payload) = Self::split_nonce_mac_and_payload(message)
            .await.map_err(ClientError::Io)?;

        let mac = crypto::box_::Tag::from_slice(mac.as_slice())
            .ok_or(ClientError::Crypto(CryptoError::MacCreationFailed))?;

        let nonce = crypto::box_::Nonce::from_slice(nonce.as_slice())
            .ok_or(ClientError::Crypto(CryptoError::NonceCreationFailed))?;

        let client_pk = crypto::box_::PublicKey::from_slice(
            key::CLIENT_PUBLIC_KEY
        ).ok_or(ClientError::Crypto(CryptoError::PublicKeyCreationFailed))?;

        let server_sk = crypto::box_::SecretKey::from_slice(
            key::SERVER_SECRET_KEY
        ).ok_or(ClientError::Crypto(CryptoError::SecretKeyCreationFailed))?;

        crypto::box_::open_detached(
            &mut payload,
            &mac,
            &nonce,
            &client_pk,
            &server_sk,
        ).map_err(|_| ClientError::Crypto(CryptoError::DecryptFailed))?;

        Ok(payload)
    }

    async fn split_nonce_mac_and_payload(
        message: &[u8],
    ) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), io::Error> {
        let mut nonce = vec![0; crypto::secretbox::NONCEBYTES];
        let mut mac = vec![0; crypto::secretbox::MACBYTES];
        let mut payload = vec![];

        let mut reader = io::Cursor::new(message);
        reader.read_exact(nonce.as_mut_slice()).await?;
        reader.read_exact(mac.as_mut_slice()).await?;
        reader.read_to_end(&mut payload).await?;

        Ok((nonce, mac, payload))
    }

    pub async fn kx_encrypt(
        &self,
        mut payload: Vec<u8>
    ) -> Result<Vec<u8>, ClientError> {
        let client_pk = crypto::box_::PublicKey::from_slice(
            key::CLIENT_PUBLIC_KEY
        ).ok_or(ClientError::Crypto(CryptoError::PublicKeyCreationFailed))?;

        let server_sk = crypto::box_::SecretKey::from_slice(
            key::SERVER_SECRET_KEY
        ).ok_or(ClientError::Crypto(CryptoError::SecretKeyCreationFailed))?;

        let bootstrap_nonce = crypto::box_::gen_nonce();
        let mac = crypto::box_::seal_detached(
            payload.as_mut_slice(),
            &bootstrap_nonce,
            &client_pk,
            &server_sk,
        );

        let message = Self::frame_kx_message(
            &bootstrap_nonce,
            mac,
            payload.as_slice(),
        ).await?;

        Ok(message)
    }

    async fn frame_kx_message(
        nonce: &crypto::box_::Nonce,
        mac: crypto::box_::Tag,
        payload: &[u8],
    ) -> Result<Vec<u8>, io::Error> {
        let mut writer = io::Cursor::new(vec![]);
        writer.write_all(&nonce.0).await?;
        writer.write_all(&mac.0).await?;
        writer.write_all(payload).await?;
        Ok(writer.into_inner())
    }
}

#[derive(Debug)]
pub struct ClientCryptoStateActiveSession {
    server_nonce: crypto::secretbox::Nonce,
    client_nonce: crypto::secretbox::Nonce,
    rx: crypto::kx::x25519blake2b::SessionKey,
    tx: crypto::kx::x25519blake2b::SessionKey,
}

impl ClientCryptoState for ClientCryptoStateActiveSession {}

impl ClientCrypto<ClientCryptoStateActiveSession> {
    pub async fn session_decrypt(
        &mut self,
        message: &[u8]
    ) -> Result<Vec<u8>, ClientError> {
        let rx = Self::session_key_to_secretbox_key(&self.state.rx)
            .map_err(ClientError::Crypto)?;

        let (mac, mut payload) = Self::split_mac_and_payload(message).await?;

        let mac = crypto::secretbox::Tag::from_slice(mac.as_slice())
            .ok_or(ClientError::Crypto(CryptoError::MacCreationFailed))?;

        crypto::secretbox::open_detached(
            &mut payload,
            &mac,
            &self.state.client_nonce,
            &rx,
        ).map_err(|_| ClientError::Crypto(CryptoError::DecryptFailed))?;

        self.state.client_nonce.increment_le_inplace();

        Ok(payload)
    }

    pub async fn session_encrypt(
        &mut self,
        payload: &mut Vec<u8>
    ) -> Result<Vec<u8>, ClientError> {
        let tx = Self::session_key_to_secretbox_key(&self.state.tx)
            .map_err(ClientError::Crypto)?;

        let mac = crypto::secretbox::seal_detached(
            payload.as_mut_slice(),
            &self.state.server_nonce,
            &tx,
        );

        let message = Self::frame_payload(mac, payload).await?;

        self.state.server_nonce.increment_le_inplace();

        Ok(message)
    }

    fn session_key_to_secretbox_key(
        key: &crypto::kx::SessionKey,
    ) -> Result<crypto::secretbox::Key, CryptoError> {
        crypto::secretbox::Key::from_slice(&key.0)
            .ok_or(CryptoError::KeyConversionFailed)
    }

    async fn frame_payload(
        mac: crypto::secretbox::Tag,
        payload: &[u8],
    ) -> Result<Vec<u8>, io::Error> {
        let mut writer = io::Cursor::new(vec![]);

        writer.write_all(&mac.0).await?;
        writer.write_all(payload).await?;

        Ok(writer.into_inner())
    }

    async fn split_mac_and_payload(
        message: &[u8]
    ) -> Result<(Vec<u8>, Vec<u8>), io::Error> {
        let mut mac = vec![0; crypto::secretbox::MACBYTES];
        let mut payload = vec![];

        let mut reader = io::Cursor::new(message);
        reader.read_exact(mac.as_mut_slice()).await?;
        reader.read_to_end(&mut payload).await?;

        Ok((mac, payload))
    }
}
