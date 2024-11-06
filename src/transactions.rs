use bitcoin::absolute::{Height, LockTime};
use bitcoin::opcodes::all::{OP_CHECKSIG, OP_IF};
use bitcoin::opcodes::OP_FALSE;
use bitcoin::script::{Builder, Instruction, PushBytesBuf};
use bitcoin::{
    Address, Amount, OutPoint, Script, ScriptBuf, Transaction, TxIn, TxOut, Witness, XOnlyPublicKey,
};

const MAX_PUSH_SIZE: usize = 520; // Bitcoin consensus rule limit

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
) -> Result<Transaction, Box<dyn std::error::Error>> {
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
