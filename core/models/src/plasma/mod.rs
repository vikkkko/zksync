pub mod account;
pub mod block;
pub mod circuit;
pub mod operations;
pub mod params;
pub mod tx;

pub use web3::types::{H256, U128, U256};

// use merkle_tree::{PedersenHasher, SparseMerkleTree};
use pairing::bn256;
use sapling_crypto::eddsa;

pub use self::account::{Account, AccountUpdate};
pub use self::operations::{DepositOp, FranklinOp, PartialExitOp, TransferOp, TransferToNewOp};
pub use self::tx::TxSignature;
use bigdecimal::BigDecimal;

pub type Engine = bn256::Bn256;
pub type Fr = bn256::Fr;

pub type AccountMap = fnv::FnvHashMap<u32, Account>;
pub type AccountUpdates = Vec<(u32, AccountUpdate)>;

pub fn apply_updates(accounts: &mut AccountMap, updates: AccountUpdates) {
    for (id, update) in updates.into_iter() {
        let updated_account = Account::apply_update(accounts.remove(&id), update);
        if let Some(account) = updated_account {
            accounts.insert(id, account);
        }
    }
}

pub fn reverse_updates(updates: &mut AccountUpdates) {
    updates.reverse();
    for (_, acc_upd) in updates.iter_mut() {
        *acc_upd = acc_upd.reversed_update();
    }
}

pub fn unpack_fee(fee: FeeAmount) -> BigDecimal {
    unimplemented!()
}

pub fn unpack_amount(amount: TokenAmount) -> BigDecimal {
    unimplemented!()
}

pub type PublicKey = eddsa::PublicKey<Engine>;
pub type PrivateKey = eddsa::PrivateKey<Engine>;

pub type TokenId = u16;

/// 3 bytes used.
pub type AccountId = u32;
pub type BlockNumber = u32;
pub type Nonce = u32;
/// 3 bytes used.
pub type TokenAmount = u32;
pub type FeeAmount = u8;