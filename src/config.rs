/// Global configuration that lives as long as the application
pub static GLOBAL_CONFIG: GlobalConfig = GlobalConfig {
    network: "http://localhost:7545",
    contract_address: "PUBLICATION CONTRACT ADDRESS",
    secret: "HARD_CODED_SECRET",
    decrypted_name: "decrypted",
};

/// Global configuration struct
pub struct GlobalConfig {
    pub network: &'static str,
    pub contract_address: &'static str,
    pub secret: &'static str,
    pub decrypted_name: &'static str,
}