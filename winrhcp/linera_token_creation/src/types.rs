use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub total_supply: u32,
    pub balances: HashMap<String, u32>,
}

#[derive(Deserialize)]
pub struct TokenRequest {
    pub name: String,
    pub symbol: String,
    pub total_supply: u32,
}
