use bitcoin::absolute::{Height, LockTime};
use bitcoin::address::Payload;
use bitcoin::block::Version;
use bitcoin::hashes::Hash;
use bitcoin::opcodes::all::{OP_CHECKSIG, OP_ENDIF, OP_IF};
use bitcoin::opcodes::{OP_FALSE, OP_TRUE};
use bitcoin::script::{Builder, Instruction, PushBytes, PushBytesBuf};
use bitcoin::{
    transaction, Address, Amount, Network, OutPoint, Script, ScriptBuf, TapTweakHash, Transaction,
    TxIn, TxOut, Witness, XOnlyPublicKey,
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

// pub fn create_inscription_script(
//     committer: &XOnlyPublicKey,
//     data: &[u8],
// ) -> Result<ScriptBuf, Box<dyn std::error::Error>> {
//     let mut script = Builder::new()
//         .push_x_only_key(&committer)
//         .push_opcode(OP_CHECKSIG);
//     // let mut script = Builder::new().push_opcode(OP_FALSE).push_opcode(OP_IF);
//     //
//     // // Add content type as a safe push
//     // // let content_type = bitcoin::script::PushBytesBuf::try_from(b"text/plain")
//     // //     .map_err(|_| "Content type too large")?;
//     // // script = script.push_slice(&content_type);
//     //
//     // // Split data into chunks of MAX_PUSH_SIZE bytes
//     // for chunk in data.chunks(MAX_PUSH_SIZE) {
//     //     let mut bytes = PushBytesBuf::new();
//     //     bytes.extend_from_slice(chunk)?;
//     //     script = script.push_slice(&bytes);
//     // }
//     //
//     // // Close protocol envelope
//     // script = script
//     //     .push_opcode(bitcoin::opcodes::all::OP_ENDIF)
//     //     .push_opcode(OP_TRUE);
//
//     Ok(script.into_script())
// }

pub fn create_inscription_script(
    committer: &XOnlyPublicKey,
    data: &[u8],
) -> Result<ScriptBuf, Box<dyn std::error::Error>> {
    let mut script = Builder::new().push_opcode(OP_FALSE).push_opcode(OP_IF);

    // Split data into chunks of MAX_PUSH_SIZE bytes
    for chunk in data.chunks(MAX_PUSH_SIZE) {
        let mut bytes = PushBytesBuf::new();
        bytes.extend_from_slice(chunk)?;
        script = script.push_slice(&bytes);
    }

    // Close protocol envelope and add key verification
    script = script
        .push_opcode(bitcoin::opcodes::all::OP_ENDIF)
        .push_x_only_key(committer) // Add the public key
        .push_opcode(OP_CHECKSIG); // Add OP_CHECKSIG instead of OP_TRUE

    Ok(script.into_script())
}

pub fn create_commit_transaction(
    script_pubkey: &ScriptBuf,
    outpoint: OutPoint,
    amount: Amount,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    // Get an unspent output to fund the inscription

    // Create commitment transaction
    let tx = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: LockTime::from(Height::MIN),
        input: vec![TxIn {
            previous_output: outpoint,
            sequence: bitcoin::Sequence::MAX,
            witness: Witness::new(),
            script_sig: ScriptBuf::new(),
        }],
        output: vec![TxOut {
            value: amount,
            script_pubkey: script_pubkey.clone(),
        }],
    };

    Ok(tx)
}

pub fn create_reveal_transaction(
    commit_tx: &Transaction,
    inscription_script: &ScriptBuf,
    tap_tweak: &TapTweakHash,
    receiver_address: &Address,
    fee: Amount,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let tap_tweak: Vec<u8> = tap_tweak.to_byte_array().to_vec();
    // let witness_data = Witness::from_slice(&[inscription_script.to_bytes(), tap_tweak]);
    // let witness_data = Witness::new();

    // Create the reveal transaction spending the commit tx
    let reveal_tx = Transaction {
        version: bitcoin::transaction::Version(2),
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: bitcoin::OutPoint::new(commit_tx.txid(), 0),
            sequence: bitcoin::Sequence::MAX,
            witness: Witness::new(),
            script_sig: ScriptBuf::new(),
        }],
        output: vec![TxOut {
            value: commit_tx.output[0].value - fee,
            script_pubkey: receiver_address.script_pubkey(),
        }],
    };

    Ok(reveal_tx)
}

pub fn extract_inscription_data(tx: &Transaction) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Get the witness data from the first input
    let witness = tx
        .input
        .get(0)
        .ok_or("No inputs found")?
        .witness
        .clone()
        .to_vec();

    // The inscription script is the second item in the witness stack
    let inscription_script = witness
        .get(1)
        .ok_or("No inscription script found in witness")?;

    let script = Script::from_bytes(inscription_script);

    let mut data = Vec::new();
    let mut in_envelope = false;

    // Iterate through script instructions
    for instruction in script.instructions() {
        match instruction? {
            Instruction::Op(bitcoin::opcodes::all::OP_IF) => {
                in_envelope = true;
            }
            // Look for OP_ENDIF to end the envelope
            Instruction::Op(bitcoin::opcodes::all::OP_ENDIF) => {
                in_envelope = false;
            }
            // Collect push data while in envelope
            Instruction::PushBytes(bytes) => {
                if in_envelope {
                    data.extend_from_slice(bytes.as_bytes());
                }
            }
            _ => {}
        }
    }

    Ok(data)
}
