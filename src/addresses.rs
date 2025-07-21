use crate::Address;

#[derive(Clone)]
pub enum KnownAddress {
    Alice,
    Bob,
    Carol,
    Dave,
    Eve,
    // Fred,
    // George,
    // Harry,
    Custom(String),
}

pub const ADDRESS_TEST_VECTORS: [&str; 5] = [
    // a Shelley address with both payment and stake parts
    "addr1q9dhugez3ka82k2kgh7r2lg0j7aztr8uell46kydfwu3vk6n8w2cdu8mn2ha278q6q25a9rc6gmpfeekavuargcd32vsvxhl7e",
    // a Shelley address with only payment part
    "addr1vx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzers66hrl8",
    // a Shelley stake address
    "stake178phkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gtcccycj5",
    // a Shelley script address
    "addr1w9jx45flh83z6wuqypyash54mszwmdj8r64fydafxtfc6jgrw4rm3",
    // a Byron address
    "37btjrVyb4KDXBNC4haBVPCrro8AQPHwvCMp3RFhhSVWwfFmZ6wwzSK6JK1hY6wHNmtrpTf1kdbva8TCneM2YsiXT7mrzT21EacHnPpz5YyUdj64na",
];

impl KnownAddress {
    pub fn everyone() -> Vec<Self> {
        vec![
            KnownAddress::Alice,
            KnownAddress::Bob,
            KnownAddress::Carol,
            KnownAddress::Dave,
            KnownAddress::Eve,
            // KnownAddress::Fred,
            // KnownAddress::George,
            // KnownAddress::Harry,
        ]
    }

    pub fn ordinal(&self) -> usize {
        match self {
            KnownAddress::Alice => 0,
            KnownAddress::Bob => 1,
            KnownAddress::Carol => 2,
            KnownAddress::Dave => 3,
            KnownAddress::Eve => 4,
            // KnownAddress::Fred => 5,
            // KnownAddress::George => 6,
            // KnownAddress::Harry => 7,
            KnownAddress::Custom(_) => 8,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            KnownAddress::Custom(addr) => addr,
            x => ADDRESS_TEST_VECTORS[x.ordinal()],
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.as_str().as_bytes().to_vec()
    }
}

impl From<KnownAddress> for Address {
    fn from(addr: KnownAddress) -> Self {
        addr.to_bytes().try_into().unwrap()
    }
}

impl From<&KnownAddress> for Address {
    fn from(addr: &KnownAddress) -> Self {
        addr.to_bytes().try_into().unwrap()
    }
}

impl From<&str> for KnownAddress {
    fn from(value: &str) -> Self {
        KnownAddress::Custom(value.to_owned())
    }
}

impl From<String> for KnownAddress {
    fn from(value: String) -> Self {
        KnownAddress::Custom(value)
    }
}

impl From<&KnownAddress> for KnownAddress {
    fn from(value: &KnownAddress) -> Self {
        value.clone()
    }
}
