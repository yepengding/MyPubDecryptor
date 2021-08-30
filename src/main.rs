use mypub_decryptor::cipher::decrypt;
use mypub_decryptor::init_cli;

/// Application entry
#[tokio::main]
async fn main() {
    // Get Config from the CLI
    let config = init_cli();

    // Do decryption
    match decrypt(&config).await {
        Ok(_) => println!("Decrypted successfully!"),
        Err(e) => println!("{}", e),
    };
}