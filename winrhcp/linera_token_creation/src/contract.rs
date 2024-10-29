use linera_sdk::Contract;
use crate::views::TokenView;
use crate::types::Token;
use serde::{Deserialize, Serialize};

pub async fn create_token(name: &str, symbol: &str, total_supply: u32) -> Result<(), String> {
    let mut view = TokenView::load().await;
    view.create_token(name, symbol, total_supply);
    view.save().await.map_err(|_| "Error saving token".to_string())
}
