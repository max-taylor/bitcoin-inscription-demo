mod actor;

use actor::Actor;
use bitcoincore_rpc::{
    bitcoin::{
        self,
        absolute::{Height, LockTime},
        opcodes::all::OP_RETURN,
        script::Builder,
        sighash::SighashCache,
        Amount, OutPoint, ScriptBuf, Transaction, TxOut, Txid, Witness,
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

    let actor = Actor::new(None);

    let initial_amount = Amount::from_sat(1_000_000);

    // Get transaction output
    let (initial_fund_txid, found_vout) = get_a_txout(&rpc, &actor.address, initial_amount.clone());

    // Create arbitrary data 1mb in size
    let arbitrary_data = [0 as u8; 32];

    let builder = Builder::new()
        .push_opcode(OP_RETURN)
        .push_slice(arbitrary_data)
        .into_script();

    let fee = Amount::from_sat(500); // 500 satoshi fee

    // Define the transaction with an OP_RETURN output
    let mut commit_tx = Transaction {
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
                value: Amount::from_sat(0),
                script_pubkey: builder,
            },
            TxOut {
                value: Amount::from_sat(100_000) - fee,
                script_pubkey: wallet_address.script_pubkey(),
            },
        ],
    };

    let mut sighash_cache = SighashCache::new(&mut commit_tx);

    let prevouts = vec![TxOut {
        script_pubkey: actor.address.script_pubkey(),
        value: initial_amount,
    }];
    let sig_hash = sighash_cache
        .taproot_key_spend_signature_hash(
            0,
            &bitcoin::sighash::Prevouts::All(&prevouts),
            bitcoin::sighash::TapSighashType::Default,
        )
        .unwrap();

    let sig = actor.sign_with_tweak(sig_hash, None);
    let witness = sighash_cache.witness_mut(0).unwrap();
    witness.push(sig.as_ref());

    // Send the transaction with witness data
    let tx = rpc.send_raw_transaction(&commit_tx)?;

    println!("Transaction sent: {}", tx);

    let transaction = rpc.get_transaction(&tx, None)?;

    dbg!(transaction);

    Ok(())
}

// fn other_test() {
//     // Find suitable UTXO with enough value
//     let unspent = rpc.list_unspent(None, None, None, None, None)?;
//     let selected_utxo = unspent
//         .iter()
//         .find(|utxo| utxo.amount > Amount::from_btc(0.001)?) // Increased amount for larger inscription
//         .ok_or("No suitable UTXO found")?;
//
//     // Create input
//     let input = bitcoin::TxIn {
//         previous_output: selected_utxo.txid.into(),
//         script_sig: Script::new(),
//         sequence: bitcoin::Sequence::MAX,
//         witness: bitcoin::Witness::new(),
//     };
//
//     // Parse destination address
//     let destination_address = Address::from_str(&config.destination_address)?;
//
//     // Calculate fees (higher for larger inscription)
//     let inscription_amount = Amount::from_btc(0.0001)?;
//     let fee = Amount::from_btc(0.0005)?; // Increased fee for larger data
//     let change_amount = selected_utxo.amount - inscription_amount - fee;
//
//     // Create outputs
//     let mut outputs = vec![
//         // Destination output
//         TxOut {
//             value: inscription_amount.to_sat(),
//             script_pubkey: destination_address.script_pubkey(),
//         },
//         // Inscription output
//         TxOut {
//             value: 0,
//             script_pubkey: script,
//         },
//     ];
//
//     // Add change output if needed
//     if change_amount > Amount::ZERO {
//         let change_address = rpc.get_new_address(None, None)?;
//         outputs.push(TxOut {
//             value: change_amount.to_sat(),
//             script_pubkey: change_address.script_pubkey(),
//         });
//     }
//
//     // Create unsigned transaction
//     let unsigned_tx = Transaction {
//         version: 2,
//         lock_time: bitcoin::PackedLockTime::ZERO,
//         input: vec![input],
//         output: outputs,
//     };
//
//     // Sign transaction
//     let signed_tx = rpc.sign_raw_transaction_with_wallet(&unsigned_tx, None, None)?;
//
//     if !signed_tx.complete {
//         return Err("Failed to sign transaction".into());
//     }
//
//     // Verify transaction size
//     let tx_size = serialize(&signed_tx.transaction?).len();
//     if tx_size > 4_000_000 {
//         // Slightly less than 4MB to account for block overhead
//         return Err(format!(
//             "Transaction size {} bytes exceeds maximum allowed size",
//             tx_size
//         )
//         .into());
//     }
//
//     // Broadcast transaction
//     let tx_id = rpc.send_raw_transaction(&signed_tx.transaction?)?;
//
//     // Ok(tx_id.to_string())
// }
