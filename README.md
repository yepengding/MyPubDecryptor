# MyPub Decryptor

Decryptor of MyPub Cipher Suite.

## Usage

```
USAGE:
    mypub-decryptor.exe [OPTIONS] --id <CID> --key <PRIVATE KEY> <FILE>

ARGS:
    <FILE>    Sets an input file path

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -i, --id <CID>               Sets the publication cid
    -k, --key <PRIVATE KEY>      Sets your private key
    -o, --output <OUTPUT DIR>    Sets your output directory
```

## Flow

1. Parse CLI argument to `Config` struct;
2. Read input file;
3. Verify file integrity on Filecoin Chain;
4. Verify payment on Ethereum chain;
5. Decrypt file with hard-coded key;
6. Output `decrypted file`.

## Development

### Build Debug

```shell
cargo build
```

### Build Release

```shell
cargo build --release
```

## MyPub Components

* [MyPub Front + Chain Linker](https://github.com/yepengding/MyPub/tree/main/ui)
* [MyPub Contract](https://github.com/yepengding/MyPub/tree/main/contracts)
* MyPub Cipher
    - [Encryptor](https://github.com/yepengding/MyPubEncryptor)
    - [Decryptor](https://github.com/yepengding/MyPubDecryptor)
* [MyPub Service](https://github.com/yepengding/MyPubService)

## Reference

- [ethers](https://docs.rs/ethers/0.5.1/ethers/)
- [tokio](https://docs.rs/tokio/1.10.1/tokio/)
- [age](https://docs.rs/age/0.6.0/age/)
- [clap](https://docs.rs/clap/3.0.0-beta.4/clap/)
