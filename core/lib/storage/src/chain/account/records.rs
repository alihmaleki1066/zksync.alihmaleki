// Workspace imports
use zksync_api_types::v02::account::EthAccountType as ApiEthAccountType;
// External imports
use sqlx::{types::BigDecimal, FromRow};

#[derive(Debug, FromRow)]
pub struct StorageAccount {
    pub id: i64,
    pub last_block: i64,
    pub nonce: i64,
    pub address: Vec<u8>,
    pub pubkey_hash: Vec<u8>,
}

#[derive(Debug, FromRow)]
pub struct StorageAccountCreation {
    pub account_id: i64,
    pub is_create: bool,
    pub block_number: i64,
    pub address: Vec<u8>,
    pub nonce: i64,
    pub update_order_id: i32,
}

#[derive(Debug, FromRow)]
pub struct StorageAccountUpdate {
    pub balance_update_id: i32,
    pub account_id: i64,
    pub block_number: i64,
    pub coin_id: i32,
    pub old_balance: BigDecimal,
    pub new_balance: BigDecimal,
    pub old_nonce: i64,
    pub new_nonce: i64,
    pub update_order_id: i32,
}

#[derive(Debug, FromRow)]
pub struct StorageAccountPubkeyUpdate {
    pub pubkey_update_id: i32,
    pub update_order_id: i32,
    pub account_id: i64,
    pub block_number: i64,
    pub old_pubkey_hash: Vec<u8>,
    pub new_pubkey_hash: Vec<u8>,
    pub old_nonce: i64,
    pub new_nonce: i64,
}

#[derive(Debug, FromRow, Clone)]
pub struct StorageBalance {
    pub account_id: i64,
    pub coin_id: i32,
    pub balance: BigDecimal,
}

#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(rename = "eth_account_type")]
pub enum EthAccountType {
    Owned,
    CREATE2,
}

impl From<EthAccountType> for ApiEthAccountType {
    fn from(account_type: EthAccountType) -> ApiEthAccountType {
        match account_type {
            EthAccountType::Owned => ApiEthAccountType::Owned,
            EthAccountType::CREATE2 => ApiEthAccountType::CREATE2,
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct StorageAccountType {
    pub account_id: i64,
    pub account_type: EthAccountType,
}
