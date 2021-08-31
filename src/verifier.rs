use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

use ethers::{
    prelude::*,
};
use ethers::signers::LocalWallet;
use k256::ecdsa::SigningKey;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::GLOBAL_CONFIG;

abigen!(
    PublicationContract,
    r#"[
        function getTokenId(string) public view returns (uint256)
        event Paid(address indexed _from, uint256 _id)
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

/// Verify whether current account has paid
/// # Arguments
/// * `private_key` - Private key of the current account
/// * `cid` - Publication CID
pub async fn verify_paid(private_key: &str, cid: &str) -> Result<bool, Box<dyn Error>> {

    // Get account from private key
    let private_key_bytes = hex::decode(private_key)?;
    let signer = SigningKey::from_bytes(private_key_bytes.as_slice())?;
    let wallet = LocalWallet::from(signer);
    let account_address = format!("{:?}", wallet.address());

    // Get provider and client
    let provider = Provider::<Http>::try_from(GLOBAL_CONFIG.network)?;
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    // Get contract
    let contract_address = Address::from_str(GLOBAL_CONFIG.contract_address)?;
    let contract = PublicationContract::new(contract_address, client.clone());

    // Get token ID
    let token_id: U256 = contract
        .get_token_id(cid.to_owned())
        .legacy()
        .call()
        .await?;

    // Get paid events
    let paid_events = contract
        .paid_filter()
        .from_block(0u64)
        .query()
        .await?;

    let paid_events = serde_json::to_string(&paid_events)?;
    let paid_events: Value = serde_json::from_str(&paid_events)?;
    let paid_events = paid_events.as_array().unwrap().clone();

    let mut r = false;
    for e in paid_events {
        let paid_event: PaidEvent = serde_json::from_value(e)?;
        if paid_event.from == account_address && paid_event.id == token_id {
            r = true;
            break;
        }
    }

    Ok(r)
}

///
#[derive(Serialize, Deserialize, Debug)]
struct PaidEvent {
    from: String,
    id: U256,
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use ethers::{prelude::*};
    use ethers::signers::LocalWallet;
    use k256::ecdsa::SigningKey;

    use crate::config::GLOBAL_CONFIG;
    use crate::verifier::verify_paid;

    #[tokio::test]
    async fn test_verify_paid() {
        let private_key = "613ab4100641eb1038fe7c3eb6cd4e9430f129583da3344b107311b7c37d5693";
        let cid = "Qm987f4e25cf77052a756533258558cca6";
        let r = verify_paid(private_key, cid).await;
        println!("{}", r.unwrap());
    }

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
