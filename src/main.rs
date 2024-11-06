mod actor;
mod rpc;
mod transactions;
mod utils;

use std::str::FromStr; // Import the SliceRandom trait

use actor::Actor;
use bitcoin::{
    hashes::Hash,
    secp256k1::{self, XOnlyPublicKey},
    taproot::{LeafVersion, TaprootBuilder},
    TapLeafHash,
};
use bitcoincore_rpc::{
    bitcoin::{self, sighash::SighashCache, Address, Amount, OutPoint, TxOut},
    RpcApi,
};
use rpc::{get_a_txout, get_rpc};
use transactions::{
    create_commit_transaction, create_inscription_script, create_reveal_transaction,
    extract_inscription_data,
};
use utils::{generate_random_chars, parse_u8_vec_to_string};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fee = Amount::from_sat(100_000);
    let secp = secp256k1::Secp256k1::new();

    let rpc = get_rpc()?;

    let actor = Actor::new(None);

    let initial_amount = Amount::from_sat(1_000_000);

    let (tx, vout) = get_a_txout(&rpc, &actor.address, initial_amount);

    // Less than 400kb works fine on the local regtest node
    let inscription_data: [u8; 397_000] = generate_random_chars::<397_000>();

    // Unspendable script
    let internal_key = XOnlyPublicKey::from_str(
        "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
    )?;

    let inscription_script = create_inscription_script(&actor.pk, &inscription_data)?;
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

    let mut commit_tx = create_commit_transaction(
        &address.script_pubkey(),
        OutPoint::new(tx, vout),
        initial_amount - fee,
    )?;

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

    let commit_tx = rpc.get_raw_transaction(&commit_txid, None)?;

    let mut reveal_tx = create_reveal_transaction(&commit_tx, &actor.address, fee)?;

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

    let transaction = rpc.get_raw_transaction(&reveal_txid, None)?;

    let extracted_inscription_data = extract_inscription_data(&transaction)?;

    assert_eq!(
        parse_u8_vec_to_string(inscription_data.to_vec()),
        parse_u8_vec_to_string(extracted_inscription_data),
        "Inscription data mismatch"
    );

    Ok(())
}
