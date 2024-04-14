use models::{player::Player, team::Team};
use sqlx::{Pool, Postgres};

use crate::repository::{db, player_repository, team_repository};

mod models;
mod repository;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    
    db::migrate();
    let pool: Pool<Postgres> = db::get_pool().await;

    let team: Team = Team { id: "Picos".to_owned(), players: vec![] };
    let player1: Player = Player { id: "0".to_owned(), name: "Pepe".to_owned(), team_id: "Picos".to_owned() };
    let player2: Player = Player { id: "1".to_owned(), name: "Ole".to_owned(), team_id: "Picos".to_owned() };
    let player3: Player = Player { id: "2".to_owned(), name: "Franziska".to_owned(), team_id: "Picos".to_owned() };

    team_repository::insert_team(team, &pool).await;
    player_repository::insert_player(player1, &pool).await;
    player_repository::insert_player(player2, &pool).await;
    player_repository::insert_player(player3, &pool).await;

    let db_team: Team = team_repository::get_team_by_id("Picos", &pool).await;

    println!("Team in database (Fetched with join) -> {:?}", db_team);
}
