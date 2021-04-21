use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Reward {
    pub timestamp: String,
    pub hash: String,
    pub gateway: String,
    pub block: u64,
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub amount: Hnt,
    pub account: String
}
