mod actor;
mod claude;
mod rpc;

use std::str::FromStr;

use actor::Actor;
use bitcoin::{
    secp256k1::{self},
    taproot::TaprootBuilder,
    XOnlyPublicKey,
};
use bitcoincore_rpc::{
    bitcoin::{self, sighash::SighashCache, Address, Amount, OutPoint, TxOut},
    RpcApi,
};
use claude::{create_commit_transaction, create_inscription_script, create_reveal_transaction};
use rpc::{get_a_txout, get_rpc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc = get_rpc()?;

    let actor = Actor::new(None);

    let initial_amount = Amount::from_sat(1_000_000);

    let (tx, vout) = get_a_txout(&rpc, &actor.address, initial_amount);

    let inscription_data = [0; 80];

    // let inscription_data = [9; 2 * 1024 * 1010]; // 2MB of zeroes for this example

    let inscription_script = create_inscription_script(&inscription_data)?;

    let internal_key = XOnlyPublicKey::from_str(
        "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
    )
    .unwrap();

    let secp = secp256k1::Secp256k1::new();

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
        initial_amount,
    )?;

    let prevouts = vec![TxOut {
        script_pubkey: actor.address.script_pubkey(),
        value: initial_amount,
    }];

    let mut sighash_cache = SighashCache::new(&mut commit_tx);

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

    let commit_txid = rpc.send_raw_transaction(&commit_tx)?;

    println!("Commit transaction broadcasted: {}", commit_txid);

    let commit_tx = rpc.get_raw_transaction(&commit_txid, None)?;

    dbg!(&commit_tx);

    let tap_tweak = taproot_tree_info.tap_tweak();

    let reveal_tx =
        create_reveal_transaction(&commit_tx, &inscription_script, &tap_tweak, &actor.address)?;

    let reveal_txid = rpc.send_raw_transaction(&reveal_tx)?;

    println!("Reveal transaction broadcasted: {}", reveal_txid);

    Ok(())
}
