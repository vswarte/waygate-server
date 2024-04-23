// TODO: actual key provider so server admins can change these
pub const CLIENT_PUBLIC_KEY: &[u8; 32] = include_bytes!("../../keys/client_public_key");
pub const SERVER_SECRET_KEY: &[u8; 32] = include_bytes!("../../keys/server_secret_key");
