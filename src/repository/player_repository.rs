use sqlx::{Pool, Postgres};

use crate::models::player::Player;

pub async fn insert_player(player: Player, pool: &Pool<Postgres>) {
    sqlx::query!(
        "
    INSERT INTO player (id, name, team_id) 
    VALUES ($1, $2, $3)
    ",
        player.id,
        player.name,
        player.team_id,
    )
    .execute(pool)
    .await
    .unwrap();
}
