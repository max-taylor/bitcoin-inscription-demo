use std::str::FromStr;

use bitcoincore_rpc::bitcoin::key::rand::{self};
use bitcoincore_rpc::bitcoin::{
    address::NetworkChecked,
    hashes::Hash,
    key::{
        rand::{rngs::StdRng, RngCore, SeedableRng},
        secp256k1::{Keypair, Secp256k1, SecretKey},
    },
    secp256k1::{schnorr::Signature, All, Message, XOnlyPublicKey},
    Address, TapNodeHash, TapSighash, TapTweakHash,
};

pub struct Actor {
    pub keypair: Keypair,
    pub address: Address<NetworkChecked>,
    pub secp: Secp256k1<All>,
    pub pk: XOnlyPublicKey,
}

impl Actor {
    pub fn new(seed: Option<u64>) -> Self {
        // Initialize the Secp256k1 context
        let secp: Secp256k1<All> = Secp256k1::new();

        let mut rng = match seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        // Generate a random 32-byte array
        // let mut rng = rand::thread_rng();
        let mut random_bytes = [0u8; 32];
        rng.fill_bytes(&mut random_bytes);

        // Convert the random bytes into a SecretKey
        let internal_secret = SecretKey::from_slice(&random_bytes).expect("Invalid secret key");

        // Create the keypair using the generated secret key
        let keypair = Keypair::from_secret_key(&secp, &internal_secret);
        let (xonly, _parity) = XOnlyPublicKey::from_keypair(&keypair);

        // Generate an address (p2tr in this case)
        let address = Address::p2tr(
            &secp,
            xonly,
            None,
            bitcoincore_rpc::bitcoin::Network::Regtest,
        );

        Actor {
            keypair,
            address,
            secp,
            pk: xonly,
        }
    }

    pub fn get_bitcoincore_rpc_address(
        &self,
    ) -> bitcoincore_rpc::bitcoin::Address<bitcoincore_rpc::bitcoin::address::NetworkChecked> {
        bitcoincore_rpc::bitcoin::Address::from_str(self.address.to_string().as_str())
            .unwrap()
            .assume_checked()
    }

    pub fn sign_with_tweak(
        &self,
        sighash: TapSighash,
        merkle_root: Option<TapNodeHash>,
    ) -> Signature {
        // Create a deterministic RNG using sighash as seed
        // let mut seed = [0u8; 32];
        // seed.copy_from_slice(sighash.as_byte_array());
        let mut deterministic_rng = StdRng::seed_from_u64(10);

        self.secp.sign_schnorr_with_rng(
            &Message::from_digest_slice(sighash.as_byte_array()).expect("should be hash"),
            &self
                .keypair
                .add_xonly_tweak(
                    &self.secp,
                    &TapTweakHash::from_key_and_tweak(self.pk, merkle_root).to_scalar(),
                )
                .unwrap(),
            &mut deterministic_rng,
        )
    }

    pub fn sign_tx(&self, sighash_bytes: &[u8; 32]) -> Signature {
        self.secp.sign_schnorr_with_rng(
            &Message::from_digest_slice(sighash_bytes).expect("should be hash"),
            &self.keypair,
            &mut rand::thread_rng(),
        )
    }
}