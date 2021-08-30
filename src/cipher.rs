use crate::Config;
use std::error::Error;

use std::fs::{File};
use std::io::{Read, Write};

use secrecy::Secret;

use crate::config::GLOBAL_CONFIG;

/// Decrypt the input file and output to disk
/// # Arguments
/// * `config` - A config struct reference
pub async fn decrypt(config: &Config) -> Result<(), Box<dyn Error>> {

    // Read encrypted file
    let mut encrypted_file = File::open(&config.file_path).unwrap();
    let mut encrypted_file_bytes = Vec::new();
    let _ = encrypted_file.read_to_end(&mut encrypted_file_bytes);
    let encrypted = &encrypted_file_bytes[..];

    // Calculate decrypted file
    let decrypted = {
        let decryptor = match age::Decryptor::new(&encrypted[..]).unwrap() {
            age::Decryptor::Passphrase(d) => d,
            _ => unreachable!(),
        };

        let mut decrypted = vec![];
        let mut reader = decryptor.decrypt(&Secret::new(GLOBAL_CONFIG.secret.to_string()), None).unwrap();
        let _ = reader.read_to_end(&mut decrypted);

        decrypted
    };

    // Write encrypted file to disk
    let mut decrypted_file = File::create(format!("{}{}", &config.output_dir, GLOBAL_CONFIG.decrypted_name))?;
    decrypted_file.write_all(&decrypted)?;
    decrypted_file.flush()?;

    Ok(())
}