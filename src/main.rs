use bitcoincore_rpc::{
    bitcoin::{
        self,
        absolute::{Height, LockTime},
        Amount, OutPoint, ScriptBuf, Transaction, Txid, Witness,
    },
    Auth, Client, RpcApi,
};

pub const WALLET_NAME: &str = "test_wallet";

fn get_a_txout(rpc: &Client, to_address: &bitcoin::Address, amount: Amount) -> (Txid, u32) {
    rpc.generate_to_address(2, &to_address)
        .unwrap_or_else(|e| panic!("Failed to generate blocks: {}", e));

    let initial_fund_txid = rpc
        .send_to_address(to_address, amount, None, None, None, None, None, None)
        .unwrap_or_else(|e| panic!("Failed to send to address: {}", e));

    let initial_fund_tx = rpc
        .get_transaction(&initial_fund_txid, None)
        .unwrap_or_else(|e| panic!("Failed to get transaction: {}", e));

    let found_vout: u32 = initial_fund_tx
        .transaction()
        .unwrap()
        .output
        .iter()
        .enumerate()
        .find(|(_, txout)| txout.script_pubkey == to_address.script_pubkey())
        .map(|(vout, _)| vout)
        .expect("Failed to find the correct vout for the to_address")
        .try_into()
        .unwrap();

    (initial_fund_txid, found_vout)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
            // println!("Error creating wallet: {}", e);
            // If creation failed, try to load existing wallet
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

    // Get a new address from the wallet
    let wallet_address = rpc.get_new_address(None, None)?.assume_checked();

    // Get transaction output
    let (initial_fund_txid, found_vout) =
        get_a_txout(&rpc, &wallet_address, Amount::from_sat(100_000));

    let commit_tx = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: LockTime::from(Height::MIN),
        input: vec![bitcoin::transaction::TxIn {
            previous_output: OutPoint {
                txid: initial_fund_txid.clone(),
                vout: found_vout,
            },
            script_sig: ScriptBuf::new(),
            sequence: bitcoin::transaction::Sequence::MAX,
            witness: Witness::new(),
        }],
        output: vec![
            TxOut {
                script_pubkey: challenge_address.script_pubkey(),
                value: Amount::from_sat(dust_limit),
            },
            TxOut {
                script_pubkey: equivocation_address.script_pubkey(),
                value: Amount::from_sat(amount - (2 * i + 1) * (fee + dust_limit)),
            },
        ],
    };

    Ok(())
}
