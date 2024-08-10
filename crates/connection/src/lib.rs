mod push;
mod crypto;
mod transport;
mod client;
mod session;

use std::sync::OnceLock;

use base64::prelude::*;
pub use push::*;
pub use crypto::*;
pub use transport::*;
pub use client::*;
pub use session::*;

pub(crate) static KX_CLIENT_PUBLIC_KEY: OnceLock<Vec<u8>> = OnceLock::new();
pub(crate) static KX_SERVER_SECRET_KEY: OnceLock<Vec<u8>> = OnceLock::new();

pub fn init_crypto(
    client_public_key: &str,
    server_secret_key: &str,
) -> Result<(), base64::DecodeError> {
    let client_public_key = BASE64_STANDARD.decode(client_public_key)?;
    KX_CLIENT_PUBLIC_KEY.set(client_public_key).expect("init_crypto called twice?");

    let server_secret_key = BASE64_STANDARD.decode(server_secret_key)?;
    KX_SERVER_SECRET_KEY.set(server_secret_key).expect("init_crypto called twice?");

    Ok(())
}
