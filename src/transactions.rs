use std::str::FromStr;

use bitcoin::absolute::{Height, LockTime};
use bitcoin::opcodes::all::{OP_CHECKSIG, OP_IF};
use bitcoin::opcodes::OP_FALSE;
use bitcoin::script::{Builder, Instruction, PushBytesBuf};
use bitcoin::taproot::{TaprootBuilder, TaprootSpendInfo};
use bitcoin::{
    secp256k1, Address, Amount, OutPoint, Script, ScriptBuf, Transaction, TxIn, TxOut, Witness,
    XOnlyPublicKey,
};

use crate::errors::InscriptionResult;

const MAX_PUSH_SIZE: usize = 520; // Bitcoin consensus rule limit

pub fn create_inscription_script(
    committer: &XOnlyPublicKey,
    data: &[u8],
) -> InscriptionResult<ScriptBuf> {
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
        .push_x_only_key(committer)
        .push_opcode(OP_CHECKSIG);

    Ok(script.into_script())
}

pub fn create_commit_transaction(
    script_pubkey: &ScriptBuf,
    outpoint: OutPoint,
    amount: Amount,
) -> InscriptionResult<Transaction> {
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
    receiver_address: &Address,
    fee: Amount,
) -> InscriptionResult<Transaction> {
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

pub fn create_commit_reveal_transactions(
    tx_to_spend: (Transaction, u32), // (txid, vout)
    commit_pk: &XOnlyPublicKey,
    reveal_receiver: &Address,
    data: &[u8],
    fee: Amount,
) -> InscriptionResult<(Transaction, Transaction, ScriptBuf, TaprootSpendInfo)> {
    let secp = secp256k1::Secp256k1::new();
    // Unspendable pub key
    let internal_key = XOnlyPublicKey::from_str(
        "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
    )?;

    let inscription_script = create_inscription_script(&commit_pk, &data)?;
    // Create Taproot tree with our inscription
    let taproot_tree_info = TaprootBuilder::new()
        .add_leaf(0, inscription_script.clone())?
        .finalize(&secp, internal_key)
        .unwrap();

    let address = Address::p2tr(
        &secp,
        internal_key,
        taproot_tree_info.merkle_root(),
        bitcoin::Network::Regtest,
    );

    let tx_to_spend_value = tx_to_spend
        .0
        .output
        .get(tx_to_spend.1 as usize)
        .unwrap()
        .value;

    let commit_tx = create_commit_transaction(
        &address.script_pubkey(),
        OutPoint::new(tx_to_spend.0.txid(), tx_to_spend.1),
        tx_to_spend_value - fee,
    )?;

    let reveal_tx = create_reveal_transaction(&commit_tx, &reveal_receiver, fee)?;

    Ok((commit_tx, reveal_tx, inscription_script, taproot_tree_info))
}

pub fn extract_inscription_data(tx: &Transaction) -> InscriptionResult<Vec<u8>> {
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
