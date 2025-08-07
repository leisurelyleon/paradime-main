use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
pub struct GenesisConfig {
    pub max_supply: u128,
    pub total_supply: u128,
    pub circulating_supply: u128,
}

pub fn load_genesis<P: AsRef<Path>>(path: P) -> GenesisConfig {
    let data = fs::read_to_string(path).expect("Failed to read genesis.json");
    serde_json::from_str(&data).expect("Invalid genesis.json format")
}
