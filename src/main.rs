mod actor;
mod constants;
mod errors;
mod rpc;
mod transactions;
mod utils;

use actor::Actor;
use bitcoin::{hashes::Hash, taproot::LeafVersion, TapLeafHash};
use bitcoincore_rpc::{
    bitcoin::{self, sighash::SighashCache, Amount, TxOut},
    RpcApi,
};
use errors::InscriptionResult;
use rpc::{get_a_txout, get_rpc};
use transactions::{create_commit_reveal_transactions, extract_inscription_data};
use utils::{generate_random_chars, parse_u8_vec_to_string};

fn main() -> InscriptionResult<()> {
    let fee = Amount::from_sat(100_000);

    let rpc = get_rpc()?;

    let actor = Actor::new(None);

    let initial_amount = Amount::from_sat(1_000_000);

    let (tx, vout) = get_a_txout(&rpc, &actor.address, initial_amount);

    // decode

    // Less than 400kb works fine on the local regtest node
    let inscription_data: [u8; 397_000] = generate_random_chars::<397_000>();

    let (mut commit_tx, mut reveal_tx, inscription_script, taproot_tree_info) =
        create_commit_reveal_transactions(
            (tx, vout),
            &actor.pk,
            &actor.address,
            &inscription_data,
            fee,
        )?;

    // --- Sending the commit transaction ---
    // In this case we are spending the output of the tx we created above  with the call
    // to `get_a_txout` function
    let prevouts = vec![TxOut {
        script_pubkey: actor.address.script_pubkey(),
        value: initial_amount,
    }];

    let mut sighash_cache = SighashCache::new(&mut commit_tx);

    let sig_hash = sighash_cache.taproot_key_spend_signature_hash(
        0,
        &bitcoin::sighash::Prevouts::All(&prevouts),
        bitcoin::sighash::TapSighashType::Default,
    )?;

    let sig = actor.sign_with_tweak(sig_hash, None);

    let witness = sighash_cache.witness_mut(0).unwrap();
    witness.push(sig.as_ref());

    let commit_txid = rpc.send_raw_transaction(&commit_tx)?;

    println!("Commit transaction broadcasted: {}", commit_txid);

    // --- Sending the reveal transaction ---

    let commit_tx = rpc.get_raw_transaction(&commit_txid, None)?;

    let mut sighash_cache = SighashCache::new(&mut reveal_tx);

    let sig_hash = sighash_cache.taproot_script_spend_signature_hash(
        0,
        &bitcoin::sighash::Prevouts::All(&commit_tx.output),
        TapLeafHash::from_script(&inscription_script, LeafVersion::TapScript),
        bitcoin::sighash::TapSighashType::Default,
    )?;

    let sig = actor.sign_tx(&sig_hash.to_byte_array());

    let inscription_script_control_block = taproot_tree_info
        .control_block(&(inscription_script.clone(), LeafVersion::TapScript))
        .expect("Cannot create inscription control block");

    let witness = sighash_cache.witness_mut(0).unwrap();
    witness.push(sig.as_ref());
    witness.push(inscription_script);
    witness.push(&inscription_script_control_block.serialize());

    let reveal_txid = rpc.send_raw_transaction(&reveal_tx)?;

    println!("Reveal transaction broadcasted: {}", reveal_txid);

    // --- Verifying the inscription data ---

    let transaction = rpc.get_raw_transaction(&reveal_txid, None)?;

    let extracted_inscription_data = extract_inscription_data(&transaction)?;

    assert_eq!(
        parse_u8_vec_to_string(inscription_data.to_vec()),
        parse_u8_vec_to_string(extracted_inscription_data),
        "Inscription data mismatch"
    );

    Ok(())
}
