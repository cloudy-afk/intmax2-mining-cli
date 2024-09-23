use intmax2_zkp::{
    ethereum_types::{address::Address, u32limb_trait::U32LimbTrait as _},
    utils::leafable::Leafable as _,
};
use mining_circuit_v1::withdrawal::simple_withraw_circuit::SimpleWithdrawalValue;

use crate::{
    external_api::contracts::events::Deposited,
    state::{keys::Key, state::State},
    utils::salt::{get_pubkey_from_private_key, get_salt_from_private_key_nonce},
};

pub fn generate_withdrawa_witness(
    state: &State,
    key: &Key,
    event: Deposited,
) -> anyhow::Result<SimpleWithdrawalValue> {
    let deposit_root = state.deposit_hash_tree.get_root();
    let deposit_index = state
        .deposit_hash_tree
        .get_index(event.deposit().hash())
        .unwrap();
    let deposit_merkle_proof = state.deposit_hash_tree.prove(deposit_index);
    let recipient = Address::from_bytes_be(key.withdrawal_address.unwrap().as_bytes());
    let pubkey = get_pubkey_from_private_key(key.deposit_private_key);
    let salt = get_salt_from_private_key_nonce(key.deposit_private_key, event.tx_nonce);
    let deposit_leaf = event.deposit();
    let withdrawal_value = SimpleWithdrawalValue::new(
        deposit_root,
        deposit_index,
        deposit_leaf,
        deposit_merkle_proof,
        recipient,
        pubkey,
        salt,
    );
    Ok(withdrawal_value)
}