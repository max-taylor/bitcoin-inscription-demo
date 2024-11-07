use bitcoin::Transaction;
use bitcoincore_rpc::{
    bitcoin::{self, Amount, Txid},
    Auth, Client, RpcApi,
};

use crate::errors::InscriptionResult;

pub const WALLET_NAME: &str = "test_wallet";

pub fn get_a_txout(
    rpc: &Client,
    to_address: &bitcoin::Address,
    amount: Amount,
) -> (Transaction, u32) {
    rpc.generate_to_address(2, &to_address)
        .unwrap_or_else(|e| panic!("Failed to generate blocks: {}", e));

    let initial_fund_txid = rpc
        .send_to_address(to_address, amount, None, None, None, None, None, None)
        .unwrap_or_else(|e| panic!("Failed to send to address: {}", e));

    let initial_fund_tx = rpc
        .get_transaction(&initial_fund_txid, None)
        .unwrap_or_else(|e| panic!("Failed to get transaction: {}", e))
        .transaction()
        .unwrap();

    let found_vout: u32 = initial_fund_tx
        .output
        .iter()
        .enumerate()
        .find(|(_, txout)| txout.script_pubkey == to_address.script_pubkey())
        .map(|(vout, _)| vout)
        .expect("Failed to find the correct vout for the to_address")
        .try_into()
        .unwrap();

    (initial_fund_tx, found_vout)
}

pub fn get_rpc() -> InscriptionResult<Client> {
    // Create a client without wallet first for wallet management
    let base_rpc = Client::new(
        "http://localhost:18443",
        Auth::UserPass("admin".to_string(), "admin".to_string()),
    )?;

    // Try to unload the wallet first (ignore errors as it might not be loaded)
    let _ = base_rpc.unload_wallet(Some(WALLET_NAME));

    // Try to create a new wallet
    match base_rpc.create_wallet(WALLET_NAME, None, None, None, None) {
        Ok(_) => println!("Created new wallet: {}", WALLET_NAME),
        Err(_e) => {
            base_rpc.load_wallet(WALLET_NAME)?;
            println!("Loaded existing wallet: {}", WALLET_NAME);
        }
    }

    // Create a new client that includes the wallet in the URL
    let wallet_url = format!("http://localhost:18443/wallet/{}", WALLET_NAME);
    let rpc = Client::new(
        &wallet_url,
        Auth::UserPass("admin".to_string(), "admin".to_string()),
    )?;

    Ok(rpc)
}
