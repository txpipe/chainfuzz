use std::{collections::HashMap, ops::Range};

use rand::Rng;
use sha2::{Digest, Sha256};

use crate::{
    Address, Asset, AssetClass, AssetPolicy, BlockHash, TxHash, TxoRef, addresses::KnownAddress,
};

pub struct Utxo {
    pub address: Address,
    pub naked_value: u64,
    pub assets: Vec<Asset>,
}

impl Utxo {
    pub fn has_asset_policy(&self, policy: &AssetPolicy) -> bool {
        self.assets
            .iter()
            .any(|asset| &asset.class.policy == policy)
    }

    pub fn has_asset_class(&self, class: &AssetClass) -> bool {
        self.assets.iter().any(|asset| &asset.class == class)
    }
}

pub trait UtxoGenerator {
    fn generate(&self, address: &KnownAddress) -> Utxo;
}

impl<F> UtxoGenerator for F
where
    F: Fn(&KnownAddress) -> Utxo,
{
    fn generate(&self, address: &KnownAddress) -> Utxo {
        self(address)
    }
}

pub fn genesis_tx_hash() -> TxHash {
    hex::decode("0000000000000000000000000000000000000000000000000000000000000000")
        .unwrap()
        .try_into()
        .unwrap()
}

pub fn slot_to_hash(slot: u64) -> BlockHash {
    let mut hasher = Sha256::new();
    hasher.update(&(slot as i32).to_le_bytes());
    let result = hasher.finalize();
    result.into()
}

// Define ChainPoint if it doesn't exist elsewhere
#[derive(Debug, Clone)]
pub enum ChainPoint {
    Origin,
    Specific(u64, BlockHash),
}

pub fn slot_to_chainpoint(slot: u64) -> ChainPoint {
    ChainPoint::Specific(slot, slot_to_hash(slot))
}

pub fn tx_sequence_to_hash(sequence: u64) -> TxHash {
    let mut hasher = Sha256::new();
    hasher.update(&sequence.to_le_bytes());
    let result = hasher.finalize();
    result.into()
}

// Helper function to replace address in a single UTXO
pub fn replace_utxo_address(utxo: Utxo, new_address: Address) -> Utxo {
    Utxo {
        address: new_address,
        naked_value: utxo.naked_value,
        assets: utxo.assets,
    }
}

pub fn replace_utxo_map_address(utxos: UtxoMap, new_address: Address) -> UtxoMap {
    utxos
        .into_iter()
        .map(|(k, v)| (k, replace_utxo_address(v, new_address.clone())))
        .collect()
}

pub fn replace_utxo_map_txhash(utxos: UtxoMap, tx_sequence: u64) -> UtxoMap {
    let new_txhash = tx_sequence_to_hash(tx_sequence);

    utxos
        .into_iter()
        .map(|(k, v)| (TxoRef::new(new_txhash, k.ordinal), v))
        .collect()
}

pub fn assert_utxo_address_and_value(utxo: &Utxo, address: impl Into<Address>, value: u64) {
    assert_eq!(utxo.address, address.into());
    assert_eq!(utxo.naked_value, value);
}

pub fn assert_utxo_map_address_and_value<A>(utxos: &UtxoMap, address: A, value: u64)
where
    A: Into<Address> + Clone,
{
    for utxo in utxos.values() {
        assert_utxo_address_and_value(utxo, address.clone(), value);
    }
}

pub fn print_utxo(txoref: &TxoRef, utxo: &Utxo) {
    let tx_hash = hex::encode(&txoref.tx_hash);
    let ordinal = txoref.ordinal;
    let address = hex::encode(&utxo.address);
    let amount = utxo.naked_value;

    println!("{tx_hash}#{ordinal} -> {address} = {amount}")
}

pub fn print_utxo_map(utxos: &UtxoMap) {
    for (txoref, utxo) in utxos.iter() {
        print_utxo(txoref, utxo);
    }
}

pub fn make_custom_utxo_map<G>(
    addresses: impl IntoIterator<Item = KnownAddress>,
    utxos_per_address: Range<u64>,
    utxo_generator: G,
) -> UtxoMap
where
    G: UtxoGenerator,
{
    let addresses = addresses.into_iter().collect::<Vec<_>>();

    let mut utxos = UtxoMap::new();

    for (tx, address) in addresses.iter().enumerate() {
        let utxo_count = rand::rng().random_range(utxos_per_address.clone());

        for ordinal in 0..utxo_count {
            let tx = tx_sequence_to_hash(tx as u64);

            let key = TxoRef::new(tx, ordinal as u16);
            let cbor = utxo_generator.generate(address);

            utxos.insert(key, cbor);
        }
    }

    utxos
}

pub fn utxo_with_random_amount(address: impl Into<Address>, amount: Range<u64>) -> Utxo {
    UtxoBuilder::new()
        .with_address(address)
        .with_random_naked_value(amount)
        .build()
}

pub const MIN_UTXO_AMOUNT: u64 = 1_111_111;

pub fn utxo_with_random_asset(
    address: impl Into<Address>,
    asset: impl Into<AssetClass>,
    asset_amount: Range<u64>,
) -> Utxo {
    UtxoBuilder::new()
        .with_naked_value(MIN_UTXO_AMOUNT)
        .with_address(address)
        .with_random_asset(asset, asset_amount)
        .build()
}

pub struct UtxoBuilder {
    address: Option<Address>,
    naked_value: Option<u64>,
    assets: Vec<Asset>,
}

impl UtxoBuilder {
    pub fn new() -> Self {
        Self {
            address: None,
            naked_value: None,
            assets: vec![],
        }
    }

    pub fn with_address(mut self, address: impl Into<Address>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn with_known_address(mut self, address: KnownAddress) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn with_naked_value(mut self, value: u64) -> Self {
        self.naked_value = Some(value);
        self
    }

    pub fn with_random_naked_value(mut self, amount: Range<u64>) -> Self {
        self.naked_value = Some(rand::rng().random_range(amount));
        self
    }

    pub fn with_asset(mut self, asset: Asset) -> Self {
        self.assets.push(asset);
        self
    }

    pub fn with_random_asset(mut self, asset: impl Into<AssetClass>, amount: Range<u64>) -> Self {
        let class: AssetClass = asset.into();
        let amount = rand::rng().random_range(amount);

        self.assets.push(Asset::new(class, amount));

        self
    }

    pub fn build(self) -> Utxo {
        Utxo {
            address: self.address.unwrap(),
            naked_value: self.naked_value.unwrap(),
            assets: self.assets,
        }
    }
}

pub struct UtxoMap(HashMap<TxoRef, Utxo>);

impl UtxoMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn find_by_address(&self, address: &Address) -> impl Iterator<Item = (&TxoRef, &Utxo)> {
        self.0.iter().filter(|(_, utxo)| utxo.address == *address)
    }

    pub fn find_by_asset_policy(
        &self,
        policy: &AssetPolicy,
    ) -> impl Iterator<Item = (&TxoRef, &Utxo)> {
        self.0
            .iter()
            .filter(move |(_, utxo)| utxo.has_asset_policy(&policy))
    }

    pub fn find_by_asset_class(
        &self,
        class: &AssetClass,
    ) -> impl Iterator<Item = (&TxoRef, &Utxo)> {
        self.0
            .iter()
            .filter(move |(_, utxo)| utxo.has_asset_class(&class))
    }
}

impl std::ops::Deref for UtxoMap {
    type Target = HashMap<TxoRef, Utxo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for UtxoMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for UtxoMap {
    type Item = (TxoRef, Utxo);
    type IntoIter = std::collections::hash_map::IntoIter<TxoRef, Utxo>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<(TxoRef, Utxo)> for UtxoMap {
    fn from_iter<T: IntoIterator<Item = (TxoRef, Utxo)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}
