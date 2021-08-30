use std::convert::TryFrom;

use ethers::{prelude::*, utils::Ganache};
use ethers::core::rand::thread_rng;
use ethers::signers::LocalWallet;
use hex_literal::hex;

use crate::config::GLOBAL_CONFIG;

pub async fn verify_paid() {


}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use ethers::{prelude::*, utils::Ganache};
    use ethers::core::rand::thread_rng;
    use ethers::signers::LocalWallet;
    use hex_literal::hex;
    use k256::ecdsa::SigningKey;

    use crate::config::GLOBAL_CONFIG;

    #[tokio::test]
    async fn signature_verification() {
        // Set private key of the first account in the test network
        // Account address: 0x8b749bCb680237aC2665d6A9aBDc78b38474A6c6
        let private_key = "eb7afb706bbc9e0dc394b37c2a6a826ab86db9e7ac925f85802e9f6fbf7eecad";

        // Get provider from running network
        let provider = Provider::<Http>::try_from(GLOBAL_CONFIG.network).unwrap();

        // Get accounts
        let accounts = provider.get_accounts().await.unwrap();

        // Get the first account
        let local_account_0 = accounts[0];

        // Authenticate the wallet by private key in byte form
        let private_key_bytes = hex::decode(private_key).unwrap();
        let signer = SigningKey::from_bytes(private_key_bytes.as_slice()).unwrap();
        let wallet = LocalWallet::from(signer);

        // Set a challenge message
        let message = "Challenge message";

        // Generate a signature signed by the first account
        let signature = wallet.sign_message(message).await.unwrap();
        println!("Produced signature {}", signature);

        // Verify the generated signature with the wallet address of the first account
        signature.verify(message, wallet.address()).unwrap();
        println!("Verified signature produced by {:?}", wallet.address());

        // Addresses should be equal
        assert_eq!(local_account_0, wallet.address());
    }
}
