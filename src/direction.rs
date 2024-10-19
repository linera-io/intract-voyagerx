use async_graphql::scalar;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

scalar!(Direction);
