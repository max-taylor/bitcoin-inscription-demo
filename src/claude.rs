use bitcoin::address::Payload;
use bitcoin::block::Version;
use bitcoin::hashes::Hash;
use bitcoin::opcodes::all::OP_IF;
use bitcoin::opcodes::OP_FALSE;
use bitcoin::script::{Builder, PushBytesBuf};
use bitcoin::{
    transaction, Address, Amount, Network, OutPoint, Script, ScriptBuf, Transaction, TxIn, TxOut,
    Witness,
};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::str::FromStr;

// pub fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Connect to Bitcoin Core RPC
//     let rpc = Client::new(
//         "http://localhost:8332",
//         Auth::UserPass("your_rpc_user".to_string(), "your_rpc_password".to_string()),
//     )?;
//
//     // The data we want to inscribe
//     let inscription_data: &[u8; 15] = b"Hello, Bitcoin!";
//
//     // Create the inscription script
//     let inscription_script = create_inscription_script(inscription_data)?;
//
//     // Create Taproot tree with our inscription
//     let (tap_script, tap_tweak) = TaprootBuilder::new()
//         .add_leaf(0, inscription_script.clone())?
//         .finalize(&secp256k1::Secp256k1::new(), bitcoin::PublicKey::new())?;
//
//     // Create the commit transaction
//     let commit_tx = create_commit_transaction(&rpc, &tap_script)?;
//
//     // Create the reveal transaction
//     let reveal_tx = create_reveal_transaction(&commit_tx, &inscription_script, &tap_tweak)?;
//
//     // Broadcast transactions
//     let commit_txid = rpc.send_raw_transaction(&commit_tx)?;
//     println!("Commit transaction broadcasted: {}", commit_txid);
//
//     // Wait for commit tx to confirm before broadcasting reveal tx
//     let reveal_txid = rpc.send_raw_transaction(&reveal_tx)?;
//     println!("Reveal transaction broadcasted: {}", reveal_txid);
//
//     Ok(())
// }

const MAX_PUSH_SIZE: usize = 520; // Bitcoin consensus rule limit

pub fn create_inscription_script(data: &[u8]) -> Result<ScriptBuf, Box<dyn std::error::Error>> {
    let mut script = Script::builder();

    // Add protocol envelope
    script = script.push_opcode(OP_FALSE).push_opcode(OP_IF);

    // Add content type as a safe push
    let content_type = bitcoin::script::PushBytesBuf::try_from(b"text/plain")
        .map_err(|_| "Content type too large")?;
    script = script.push_slice(&content_type);

    // Split data into chunks of MAX_PUSH_SIZE bytes
    for chunk in data.chunks(MAX_PUSH_SIZE) {
        let mut bytes = PushBytesBuf::new();
        bytes.extend_from_slice(chunk)?;
        // Convert each chunk into PushBytesBuf
        // let push_bytes = PushBytesBuf::from_slice(chunk)
        //     .map_err(|_| format!("Chunk size {} exceeds maximum push size", chunk.len()))?;
        script = script.push_slice(&bytes);
    }

    // Close protocol envelope
    script = script.push_opcode(bitcoin::opcodes::all::OP_ENDIF);

    Ok(script.into_script())
}

pub fn create_commit_transaction(
    tap_script: &ScriptBuf,
    outpoint: OutPoint,
    amount: Amount,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let fee = Amount::from_sat(1000);
    // Get an unspent output to fund the inscription

    // Create commitment transaction
    let tx = Transaction {
        version: transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: outpoint,
            sequence: bitcoin::Sequence::MAX,
            witness: Witness::new(),
            script_sig: ScriptBuf::new(),
        }],
        output: vec![TxOut {
            value: amount - fee, // Subtract fee
            script_pubkey: tap_script.clone(),
        }],
    };

    Ok(tx)
}
//
// fn create_reveal_transaction(
//     commit_tx: &Transaction,
//     inscription_script: &Script,
//     tap_tweak: &[u8],
// ) -> Result<Transaction, Box<dyn std::error::Error>> {
//     // Create the reveal transaction spending the commit tx
//     let reveal_tx = Transaction {
//         version: 2,
//         lock_time: bitcoin::absolute::LockTime::ZERO,
//         input: vec![TxIn {
//             previous_output: bitcoin::OutPoint::new(commit_tx.txid(), 0),
//             sequence: bitcoin::Sequence::MAX,
//             witness: Witness::from_vec(vec![inscription_script.to_bytes(), tap_tweak.to_vec()]),
//             script_sig: ScriptBuf::new(),
//         }],
//         output: vec![TxOut {
//             value: commit_tx.output[0].value - 1000, // Subtract fee
//             script_pubkey: Address::from_str("bc1...")? // Add recipient address
//                 .script_pubkey(),
//         }],
//     };
//
//     Ok(reveal_tx)
// }
