use clap::{App, Arg};

mod config;
pub mod cipher;
pub mod verifier;

/// Config struct that holds decryption information
#[derive(Debug)]
pub struct Config {
    pub file_path: String,
    pub cid: String,
    pub private_key: String,
    pub output_dir: String,
}

/// Initialize CLI
pub fn init_cli() -> Config {
    let matches = App::new("MyPub Decryptor")
        .version("0.1.0")
        .author("Yepeng Ding <yepengding@g.ecc.u-tokyo.ac.jp>")
        .about("Decryption module of MyPub Cipher Suite")
        .arg(Arg::new("FILE")
            .about("Sets an input file path")
            .required(true)
            .index(1))
        .arg(Arg::new("cid")
            .short('i')
            .long("id")
            .value_name("CID")
            .about("Sets the publication cid")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("private_key")
            .short('k')
            .long("key")
            .value_name("PRIVATE KEY")
            .about("Sets your private key")
            .takes_value(true)
            .required(true))
        .arg(Arg::new("output_dir")
            .short('o')
            .long("output")
            .value_name("OUTPUT DIR")
            .about("Sets your output directory")
            .takes_value(true))
        .get_matches();

    let mut config = Config {
        file_path: String::from(""),
        cid: String::from(""),
        private_key: String::from(""),
        output_dir: String::from("./"),
    };

    if let Some(file_path) = matches.value_of("FILE") {
        config.file_path = String::from(file_path);
    }

    if let Some(cid) = matches.value_of("CID") {
        config.cid = String::from(cid);
    }

    if let Some(private_key) = matches.value_of("private_key") {
        config.private_key = String::from(private_key);
    }

    if let Some(output_dir) = matches.value_of("output_dir") {
        config.output_dir = String::from(output_dir);
    }

    config
}