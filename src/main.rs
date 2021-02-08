
use bitcoin::hashes::{self, Hash};

use std::env;
use std::str::FromStr;

fn main() {
    for arg in env::args() {
        let mut start = 0;
        let mut end = arg.len();
        for (n, ch) in arg.char_indices() {
            if n == start + 1 && !ch.is_alphanumeric() {
                start = n;
            } else if ch.is_alphanumeric() {
                end = n + 1;
            }
        }
        let arg = &arg[start..end];

        if let Ok(mut key) = bitcoin::PrivateKey::from_str(&arg) {
            key.network = bitcoin::Network::Bitcoin;
            println!("bitcoin: {}", key);
            key.network = bitcoin::Network::Testnet;
            println!("testnet: {}", key);
        }

        if let Ok(mut addr) = bitcoin::Address::from_str(&arg) {
            addr.network = bitcoin::Network::Bitcoin;
            println!("bitcoin: {}", addr);
            addr.network = bitcoin::Network::Testnet;
            println!("testnet: {}", addr);
            let elem_payload = match addr.payload {
                bitcoin::util::address::Payload::PubkeyHash(hash) => elements::address::Payload::PubkeyHash(hash.as_hash().into()),
                bitcoin::util::address::Payload::ScriptHash(hash) => elements::address::Payload::ScriptHash(hash.as_hash().into()),
                bitcoin::util::address::Payload::WitnessProgram { version, program } => elements::address::Payload::WitnessProgram { version, program },
            };
            let mut elem_addr = elements::Address {
                params: &elements::AddressParams::LIQUID,
                payload: elem_payload,
                blinding_pubkey: None,
            };
            println!("liquid: {}", elem_addr);
            elem_addr.params = &elements::AddressParams::ELEMENTS;
            println!("elements: {}", elem_addr);

        }

        if let Ok(hash) = hashes::sha256d::Hash::from_str(&arg) {
            println!("hash: {}", hash);
            let r = hashes::sha256::Hash::from_inner(*hash.as_inner());
            println!("revr: {}", r);
        }

        if let Ok(hash) = hashes::sha1::Hash::from_str(&arg) {
            println!("hash: {}", hash);
            println!("   6: {:.06x}", hash);
            println!("   8: {:.08x}", hash);
            println!("  10: {:.10x}", hash);
            println!("  12: {:.12x}", hash);
        }
    }

}
