use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

/// This function takes username and password and hashes them.
pub fn hash_username_and_password(username: String, password: String) {}

/// This function takes username and password and returns a private key.
/// The private key is used to authenticate the user.
pub fn get_private_key(username: String, password: String) {
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(0);
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let dec_data = priv_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
}
