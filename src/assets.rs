use crate::{AssetClass, AssetPolicy};

pub enum KnownAsset {
    Hosky,
    Snek,
    NikePig,
    Custom(&'static str, &'static str),
}

impl KnownAsset {
    pub fn policy_hex(&self) -> &str {
        match self {
            KnownAsset::Hosky => "a0028f350aaabe0545fdcb56b039bfb08e4bb4d8c4d7c3c7d481c235",
            KnownAsset::Snek => "279c909f348e533da5808898f87f9a14bb2c3dfbbacccd631d927a3f",
            KnownAsset::NikePig => "c881c20e49dbaca3ff6cef365969354150983230c39520b917f5cf7c",
            KnownAsset::Custom(policy, _) => policy,
        }
    }

    pub fn ticker(&self) -> &str {
        match self {
            KnownAsset::Hosky => "HOSKY",
            KnownAsset::Snek => "SNEK",
            KnownAsset::NikePig => "NIKEPIG",
            KnownAsset::Custom(_, name) => name,
        }
    }

    pub fn name(&self) -> &[u8] {
        self.ticker().as_bytes()
    }

    pub fn policy(&self) -> AssetPolicy {
        let decoded = hex::decode(self.policy_hex()).unwrap();
        decoded.try_into().unwrap()
    }
}

impl From<KnownAsset> for AssetClass {
    fn from(asset: KnownAsset) -> Self {
        Self {
            policy: asset.policy(),
            name: asset.name().to_vec(),
        }
    }
}
