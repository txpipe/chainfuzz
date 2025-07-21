pub mod addresses;
pub mod assets;
pub mod cardano;
pub mod utxos;

pub const HASH_SIZE: usize = 32;
pub const ADDRESS_SIZE: usize = 32;
pub const ASSET_POLICY_SIZE: usize = 28;

pub type BlockHash = [u8; HASH_SIZE];
pub type TxHash = [u8; HASH_SIZE];
pub type Address = Vec<u8>;
pub type AssetPolicy = [u8; ASSET_POLICY_SIZE];
pub type AssetName = Vec<u8>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TxoRef {
    pub tx_hash: TxHash,
    pub ordinal: u16,
}

impl TxoRef {
    pub fn new(tx_hash: TxHash, ordinal: u16) -> Self {
        Self { tx_hash, ordinal }
    }
}

pub use utxos::UtxoMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetClass {
    pub policy: AssetPolicy,
    pub name: AssetName,
}

impl AssetClass {
    pub fn new(policy: AssetPolicy, name: AssetName) -> Self {
        Self { policy, name }
    }
}

pub struct Asset {
    pub class: AssetClass,
    pub amount: u64,
}

impl Asset {
    pub fn new(class: AssetClass, amount: u64) -> Self {
        Self { class, amount }
    }
}

pub use utxos::Utxo;
