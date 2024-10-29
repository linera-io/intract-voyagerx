use linera_sdk::View;
use crate::types::Token;
use std::collections::HashMap;

#[derive(View)]
pub struct TokenView {
    pub tokens: HashMap<String, Token>,
}

impl TokenView {
    pub fn create_token(&mut self, name: &str, symbol: &str, total_supply: u32) {
        let token = Token {
            name: name.to_string(),
            symbol: symbol.to_string(),
            total_supply,
            balances: HashMap::new(),
        };
        self.tokens.insert(name.to_string(), token);
    }
}
