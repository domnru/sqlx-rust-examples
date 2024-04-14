use sqlx::FromRow;

use super::player::Player;

#[derive(Debug, FromRow)]
pub struct Team {
    pub id: String,
    pub players: Vec<Player>,
}