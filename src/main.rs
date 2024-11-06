mod actor;
mod claude;
mod rpc;

use std::str::FromStr;

use actor::Actor;
use bitcoin::{
    hashes::serde::Serialize,
    secp256k1::{self},
    taproot::{LeafVersion, TaprootBuilder},
    TapLeafHash, XOnlyPublicKey,
};
use bitcoincore_rpc::{
    bitcoin::{self, sighash::SighashCache, Address, Amount, OutPoint, TxOut},
    RpcApi,
};
use claude::{create_commit_transaction, create_inscription_script, create_reveal_transaction};
use rpc::{get_a_txout, get_rpc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fee = Amount::from_sat(1000);
    let secp = secp256k1::Secp256k1::new();

    let rpc = get_rpc()?;

    let actor = Actor::new(None);

    let initial_amount = Amount::from_sat(1_000_000);

    let (tx, vout) = get_a_txout(&rpc, &actor.address, initial_amount);

    let inscription_data = [0; 80];

    // let inscription_data = [9; 2 * 1024 * 1010]; // 2MB of zeroes for this example

    let internal_key = XOnlyPublicKey::from_str(
        "93c7378d96518a75448821c4f7c8f4bae7ce60f804d03d1f0628dd5dd0f5de51",
    )
    .unwrap();

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

    let mut reveal_tx =
        create_reveal_transaction(&commit_tx, &inscription_script, &tap_tweak, &actor.address)?;

    let mut sighash_cache = SighashCache::new(&mut reveal_tx);

    let sig_hash = sighash_cache
        .taproot_script_spend_signature_hash(
            0,
            &bitcoin::sighash::Prevouts::All(&commit_tx.output),
            TapLeafHash::from_script(&inscription_script, LeafVersion::TapScript),
            bitcoin::sighash::TapSighashType::Default,
        )
        .unwrap();

    let sig = actor.sign_with_tweak(sig_hash, None);

    let inscription_script_control_block = taproot_tree_info
        .control_block(&(inscription_script.clone(), LeafVersion::TapScript))
        .expect("Cannot create inscription control block");

    let witness = sighash_cache.witness_mut(0).unwrap();
    // witness.push(sig.as_ref());
    witness.push(inscription_script);
    witness.push(&inscription_script_control_block.serialize());

    let reveal_txid = rpc.send_raw_transaction(&reveal_tx)?;

    println!("Reveal transaction broadcasted: {}", reveal_txid);

    Ok(())
}

// pub fn fill_response_tx_with_witness_for_equivocation(
//     response_tx: &mut Transaction,
//     challenge_tx: &Transaction,
//     verifier: &Actor,
//     equivocation_taproot_info: &TaprootSpendInfo,
//     hashes: HashTuple,
//     preimages: PreimageTuple,
// ) {
//     let equivocation_script = generate_anti_contradiction_script(hashes, verifier.pk);
//     let equivocation_control_block = equivocation_taproot_info
//         .control_block(&(equivocation_script.clone(), LeafVersion::TapScript))
//         .expect("Cannot create equivocation control block");
//
//     let mut sighash_cache = SighashCache::new(response_tx);
//
//     let sig_hash = sighash_cache
//         .taproot_script_spend_signature_hash(
//             0,
//             &bitcoin::sighash::Prevouts::All(&[challenge_tx.output[1].clone()]),
//             TapLeafHash::from_script(&equivocation_script, LeafVersion::TapScript),
//             bitcoin::sighash::TapSighashType::Default,
//         )
//         .unwrap();
//
//     let equivocation_sig = verifier.sign_tx(&sig_hash.to_byte_array());
//
//     // Equivocation witness data
//     let witness = sighash_cache.witness_mut(0).unwrap();
//     witness.push(equivocation_sig.as_ref());
//     witness.push(preimages.one.unwrap());
//     witness.push(preimages.zero.unwrap());
//     witness.push(equivocation_script);
//     witness.push(&equivocation_control_block.serialize());
// }
