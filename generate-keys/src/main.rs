use base64::prelude::*;

use libsodium_sys::{crypto_box_PUBLICKEYBYTES, crypto_box_SECRETKEYBYTES, crypto_box_keypair};

fn main() {
    let (client_public, client_secret) = generate_keypair();
    let (server_public, server_secret) = generate_keypair();

    println!("# Pass this to your server's env");
    println!(
        "WAYGATE_CLIENT_PUBLIC_KEY={}",
        BASE64_STANDARD.encode(client_public)
    );
    println!(
        "WAYGATE_SERVER_SECRET_KEY={}",
        BASE64_STANDARD.encode(server_secret)
    );
    println!();
    println!("# Add this to your clients's env");
    println!(
        "WAYGATE_CLIENT_SECRET_KEY={}",
        BASE64_STANDARD.encode(client_secret)
    );
    println!(
        "WAYGATE_SERVER_PUBLIC_KEY={}",
        BASE64_STANDARD.encode(server_public)
    );
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
