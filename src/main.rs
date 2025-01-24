use bip39::{Mnemonic, MnemonicType, Language, Seed};
use sha2::{Sha256, Digest};
use num_bigint::BigUint;
use sodiumoxide::crypto::{sign::ed25519};
use std::fs::{OpenOptions};
use std::io::Write;

fn generate_passphrase() -> (String, Seed) {
    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let phrase = mnemonic.phrase().to_string();
    let seed = Seed::new(&mnemonic, "");
    (phrase, seed)
}

fn create_hash_from_passphrase(seed: &Seed) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    hasher.finalize().to_vec()
}

fn create_keypair_from_passphrase(hash: &[u8]) -> (ed25519::PublicKey, ed25519::SecretKey) {
    sodiumoxide::init().expect("Failed to initialize sodiumoxide");
    let seed = ed25519::Seed::from_slice(hash).expect("Invalid hash length; must be 32 bytes");
    ed25519::keypair_from_seed(&seed)
}

pub fn create_address_from_public_key(public_key: &ed25519::PublicKey) -> String {
    // Hash the public key using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(public_key.as_ref());
    let public_key_hash = hasher.finalize();

    // Extract the last 8 bytes in reverse order
    let mut temp = [0u8; 8];
    for i in 0..8 {
        temp[i] = public_key_hash[7 - i];
    }

    // Convert the 8-byte buffer to a big integer
    let address_number = BigUint::from_bytes_be(&temp);

    // Return the address as a string prefixed with "U"
    format!("U{}", address_number)
}

fn append_to_file(file_path: &str, content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() {
    loop {
        // Generate passphrase and seed
        let (phrase, seed) = generate_passphrase();
        // println!("Passphrase: {}", phrase);

        // Create hash from seed
        let hash = create_hash_from_passphrase(&seed);
        // println!("Hash: {:X?}", hash);

        // Create keypair from hash
        let (public_key, private_key) = create_keypair_from_passphrase(&hash);
        // println!("Public Key: {:?}", public_key);
        // println!("Private Key: {:?}", private_key);

        // Create ADM address
        let adm_address = create_address_from_public_key(&public_key);
        // println!("ADM Address: {}", adm_address);


        if adm_address.len() <= 14 {
            println!("ADM Address: {}", adm_address);
            let content = adm_address + ":" + &phrase + "\n";
            append_to_file("accounts.csv", &content);
        }
    }
}
