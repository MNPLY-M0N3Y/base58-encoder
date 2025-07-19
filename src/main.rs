use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(name = "solana-base58-cli")]
#[command(about = "A CLI tool to encode/decode Base58 strings like Solana")]
#[command(version = "0.1.0")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        #[arg(short, long)]
        input: String,
        #[arg(long)]
        hex: bool,
    },

    Decode {
        #[arg(short, long)]
        input: String,
        #[arg(long)]
        hex: bool,
    },

    WalletToBase58 {
        #[arg(short, long)]
        file: String,
    },

    Base58ToWallet {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode { input, hex } => {
            let bytes = if hex {
                match hex::decode(&input) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        eprintln!("❌ Error decoding hex: {}", e);
                        return;
                    }
                }
            } else {
                input.into_bytes()
            };

            let encoded = bs58::encode(&bytes).into_string();
            println!("✅ Base58 encoded: {}", encoded);
        }

        Commands::Decode { input, hex } => match bs58::decode(&input).into_vec() {
            Ok(decoded) => {
                if hex {
                    println!("✅ Decoded (hex): {}", hex::encode(&decoded));
                } else {
                    match String::from_utf8(decoded.clone()) {
                        Ok(string) => println!("✅ Decoded (string): {}", string),
                        Err(_) => {
                            println!("✅ Decoded (bytes): {:?}", decoded);
                            println!("✅ Decoded (hex): {}", hex::encode(&decoded));
                        }
                    }
                }
            }
            Err(e) => eprintln!("❌ Error decoding Base58: {}", e),
        },

        Commands::WalletToBase58 { file } => match fs::read_to_string(&file) {
            Ok(content) => match serde_json::from_str::<Vec<u8>>(&content) {
                Ok(wallet_bytes) => {
                    let base58_key = bs58::encode(&wallet_bytes).into_string();
                    println!("✅ Base58 private key: {}", base58_key);
                    println!("📄 Wallet byte array length: {} bytes", wallet_bytes.len());
                }
                Err(e) => eprintln!("❌ Error parsing JSON: {}", e),
            },
            Err(e) => eprintln!("❌ Error reading file: {}", e),
        },

        Commands::Base58ToWallet { input, output } => match bs58::decode(&input).into_vec() {
            Ok(decoded) => {
                let json_output =
                    serde_json::to_string_pretty(&decoded).expect("Failed to serialize to JSON");

                if let Some(output_file) = output {
                    match fs::write(&output_file, &json_output) {
                        Ok(_) => println!("✅ Wallet saved to: {}", output_file),
                        Err(e) => eprintln!("❌ Error writing file: {}", e),
                    }
                } else {
                    println!("✅ Wallet format (u8 array): {}", json_output);
                }
                println!("📄 Decoded byte array length: {} bytes", decoded.len());
            }
            Err(e) => eprintln!("❌ Error decoding Base58: {}", e),
        },
    }
}
