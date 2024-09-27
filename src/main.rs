use std::env;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
use std::fs::File;
use std::io::prelude::*;
use serde_json;

/// Function that writes de information in "data" variable into a file in the path "path"
/// 
///  ```
///     write("./file.json", "\{\"var1\": 1\}");
///  ```
fn write(path: String, data: String){
    match File::create(path) {
        Ok(mut file) => {
            match file.write(data.as_bytes()) {
                Ok(_) => println!("File saved"),
                Err(_) => println!("Error saving file"),
            };
        },
        Err(s) => {
            println!("{}", s);
        }
    }
}

/// Main function
fn main() {
    // get arguments from command line
    let args: Vec<String> = env::args().collect();
    let mut block: bool = false;
    let mut slot: u64 = 0;
    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::JsonParsed),
        transaction_details: Some(TransactionDetails::Full),
        rewards: Some(true),
        commitment: None,
        max_supported_transaction_version: Some(0),
    };
    //get number of slot
    for arg in args {
        if block {
            slot = arg.parse::<u64>().unwrap();
            block = false;
        }
        if arg == "--block".to_string() {
            block = true;
        }
    }

    // check that slot number different from 0
    if slot != 0 {
        let http_client: RpcClient = RpcClient::new("https://api.mainnet-beta.solana.com");
        match http_client.get_block_with_config(slot, config) {
            Ok(s) => {
                write(format!("./{}.json", slot), format!("{}", match serde_json::to_string_pretty(&s) {
                    Ok(t) => t,
                    Err(_) => "".to_string(),
                }));
            }
            Err(_) => println!("Coudn't get block"),
        }
    } else {
        println!("Block coudn't be 0");
    }

}