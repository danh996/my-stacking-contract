use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    pub reward_numerator: u32,
    pub reward_denumerator: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            reward_numerator: 715,
            reward_denumerator: 1000000000,
        } //reward per block
    }
}

//APR 15% = {token * 715 / 100000000} * total block
