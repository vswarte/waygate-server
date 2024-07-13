use base64::prelude::*;

use libsodium_sys::{
    crypto_box_keypair,
    crypto_box_PUBLICKEYBYTES,
    crypto_box_SECRETKEYBYTES,
};

fn main() {
    let (client_public, client_secret) = generate_keypair();
    let (server_public, server_secret) = generate_keypair();

    println!("# Add this to server's general.toml");
    println!("client_public_keu = \"{}\"", BASE64_STANDARD.encode(client_public));
    println!("server_secret_key = \"{}\"", BASE64_STANDARD.encode(server_secret));
    println!();
    println!("# Add this to clients's waygate.toml");
    println!("client_secret_keu = \"{}\"", BASE64_STANDARD.encode(client_secret));
    println!("server_public_key = \"{}\"", BASE64_STANDARD.encode(server_public));
}

fn generate_keypair() -> (
    [u8; crypto_box_PUBLICKEYBYTES as usize],
    [u8; crypto_box_SECRETKEYBYTES as usize],
) {
    let mut pk = [0u8; crypto_box_PUBLICKEYBYTES as usize];
    let mut sk = [0u8; crypto_box_SECRETKEYBYTES as usize];

    let result = unsafe { crypto_box_keypair(pk.as_mut_ptr(), sk.as_mut_ptr()) };
    if result != 0x0 {
        panic!("Could not generate keypair");
    }

    (pk, sk)
}
