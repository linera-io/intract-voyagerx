#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use self::state::Game2048;
use async_graphql::{EmptySubscription, Object, Schema, SimpleObject};
use game2048::{Direction, Game, Operation};
use linera_sdk::{base::WithServiceAbi, bcs, views::View, Service, ServiceRuntime};

pub struct Game2048Service {
    state: Arc<Game2048>,
    // runtime: Arc<Mutex<ServiceRuntime<Self>>>,
}

linera_sdk::service!(Game2048Service);

impl WithServiceAbi for Game2048Service {
    type Abi = game2048::Game2048Abi;
}

impl Service for Game2048Service {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = Game2048::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Game2048Service {
            state: Arc::new(state),
            // runtime: Arc::new(Mutex::new(runtime)),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        let schema = Schema::build(
            QueryRoot {
                state: self.state.clone(),
                // runtime: self.runtime.clone(),
            },
            MutationRoot,
            EmptySubscription,
        )
        .finish();
        schema.execute(query).await
    }
}

struct QueryRoot {
    state: Arc<Game2048>,
    // runtime: Arc<Mutex<ServiceRuntime<Game2048Service>>>,
}

#[derive(SimpleObject)]
struct GameState {
    game_id: u16,
    board: [[u16; 4]; 4],
    is_ended: bool,
    score: u64,
}

#[Object]
impl QueryRoot {
    async fn game(&self, game_id: u16) -> Option<GameState> {
        if let Ok(Some(game)) = self.state.games.try_load_entry(&game_id).await {
            let game_state = GameState {
                game_id: *game.game_id.get(),
                board: Game::convert_to_matrix(*game.board.get()),
                is_ended: *game.is_ended.get(),
                score: *game.score.get(),
            };
            Some(game_state)
        } else {
            None
        }
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn new_game(&self, seed: Option<u16>) -> Vec<u8> {
        let seed = seed.unwrap_or(0);
        bcs::to_bytes(&Operation::NewGame { seed }).unwrap()
    }

    async fn make_move(&self, game_id: u16, direction: Direction) -> Vec<u8> {
        let operation = Operation::MakeMove { game_id, direction };
        bcs::to_bytes(&operation).unwrap()
    }
}
