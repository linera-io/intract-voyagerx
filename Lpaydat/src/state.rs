use async_graphql::{scalar, SimpleObject};
use linera_sdk::views::{
    linera_views, CollectionView, RegisterView, RootView, View, ViewStorageContext,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum GameStatus {
    #[default]
    Active,
    Ended,
}
scalar!(GameStatus);

#[derive(View, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct GameState {
    pub game_id: RegisterView<u16>,
    pub board: RegisterView<u64>,
    pub score: RegisterView<u64>,
    pub is_ended: RegisterView<bool>,
}

#[derive(RootView, SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Game2048 {
    pub games: CollectionView<u16, GameState>,
    // leaderboard
}
