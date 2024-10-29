#![cfg_attr(target_arch = "wasm32", no_main)]

// mod game;
mod state;

use std::str::FromStr;

use linera_sdk::{
    base::{ChainId, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime,
};

use self::state::Game2048;
use game2048::{gen_range, Game, Message, Operation};

pub struct Game2048Contract {
    state: Game2048,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(Game2048Contract);

impl WithContractAbi for Game2048Contract {
    type Abi = game2048::Game2048Abi;
}

impl Contract for Game2048Contract {
    type Message = Message;
    type Parameters = ();
    type InstantiationArgument = u16;

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Game2048::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Game2048Contract { state, runtime }
    }

    async fn instantiate(&mut self, seed: Self::InstantiationArgument) {
        self.runtime.application_parameters();

        // Initialize a default game entry if it doesn't exist
        let game_id = seed; // Example game ID
        if self
            .state
            .games
            .load_entry_or_insert(&game_id)
            .await
            .is_err()
        {
            let game = self.state.games.load_entry_mut(&game_id).await.unwrap();
            game.game_id.set(game_id);
            game.board.set(0); // Set a default board value, e.g., an empty board
        }
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::NewGame { seed } => {
                let seed = self.get_seed(seed);
                let new_board = Game::new(seed).board;
                let game = self.state.games.load_entry_mut(&seed).await.unwrap();

                game.game_id.set(seed);
                game.board.set(new_board);

                self.send_message(seed, new_board, 0, false);
            }
            Operation::EndGame { game_id } => {
                let board = self.state.games.load_entry_mut(&game_id).await.unwrap();
                board.is_ended.set(true);
            }
            Operation::MakeMove { game_id, direction } => {
                let seed = self.get_seed(0);
                let board = self.state.games.load_entry_mut(&game_id).await.unwrap();

                let is_ended = board.is_ended.get();
                if !is_ended {
                    let mut game = Game {
                        board: *board.board.get(),
                        seed,
                    };

                    let new_board = Game::execute(&mut game, direction);
                    let is_ended = Game::is_ended(new_board);
                    let score = Game::score(new_board);

                    board.board.set(new_board);
                    board.score.set(score);
                    if is_ended {
                        board.is_ended.set(true);
                    }

                    self.send_message(game_id, new_board, score, is_ended);
                }
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl Game2048Contract {
    fn get_seed(&mut self, init_seed: u16) -> u16 {
        if init_seed != 0 {
            init_seed
        } else {
            let block_height = self.runtime.block_height().to_string();
            gen_range(&block_height, 0, u16::MAX)
        }
    }

    fn send_message(&mut self, game_id: u16, board: u64, score: u64, is_ended: bool) {
        let chain_id =
            ChainId::from_str("256e1dbc00482ddd619c293cc0df94d366afe7980022bb22d99e33036fd465dd")
                .unwrap();
        self.runtime
            .prepare_message(Message::Game {
                game_id,
                board,
                score,
                is_ended,
            })
            .send_to(chain_id);
    }
}
