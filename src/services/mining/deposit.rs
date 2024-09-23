use ethers::types::U256;
use intmax2_zkp::{
    common::deposit::get_pubkey_salt_hash, ethereum_types::u32limb_trait::U32LimbTrait as _,
};

use crate::{
    external_api::contracts::{int1::get_int1_contract_with_signer, utils::get_account_nonce},
    services::contracts::handle_contract_call,
    state::{keys::Key, state::State},
    utils::salt::{get_pubkey_from_private_key, get_salt_from_private_key_nonce},
};

pub async fn deposit_task(_state: &State, key: &Key, mining_unit: U256) -> anyhow::Result<()> {
    let deposit_address = key.deposit_address;
    let nonce = get_account_nonce(deposit_address).await?;
    let salt = get_salt_from_private_key_nonce(key.deposit_private_key, nonce);
    let pubkey = get_pubkey_from_private_key(key.deposit_private_key);
    let pubkey_salt_hash: [u8; 32] = get_pubkey_salt_hash(pubkey, salt)
        .to_bytes_be()
        .try_into()
        .unwrap();

    let deposit_address = key.deposit_address;
    let int1 = get_int1_contract_with_signer(key.deposit_private_key).await?;
    let mut tx = int1
        .deposit_native_token(pubkey_salt_hash)
        .value(mining_unit);
    tx.tx.set_nonce(nonce);

    handle_contract_call(tx, deposit_address, "deposit", "deposit").await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::test::{get_dummy_keys, get_dummy_state};

    #[tokio::test]
    async fn test_deposit() {
        let state = get_dummy_state().await;
        let dummy_key = get_dummy_keys().await;

        let mining_uint = 100_000_000_000_000_000u128.into();
        super::deposit_task(&state, &dummy_key, mining_uint)
            .await
            .unwrap();
    }
}