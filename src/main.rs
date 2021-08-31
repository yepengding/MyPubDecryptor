use mypub_decryptor::cipher::decrypt;
use mypub_decryptor::init_cli;
use mypub_decryptor::verifier::{verify_integrity, verify_paid};

/// Application entry
#[tokio::main]
async fn main() {
    // Get Config from the CLI
    let config = init_cli();
    let verified_integrity = match verify_integrity(&config.cid).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            false
        }
    };

    if !verified_integrity {
        eprintln!("Encrypted file has been tampered");
        return;
    }

    let verified_paid = match verify_paid(&config.private_key, &config.cid).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            false
        }
    };

    if verified_paid {
        // Do decryption
        match decrypt(&config).await {
            Ok(_) => println!("Decrypted successfully!"),
            Err(e) => eprintln!("{}", e),
        };
    } else {
        eprintln!("Failed to verify payment information!");
    }
}