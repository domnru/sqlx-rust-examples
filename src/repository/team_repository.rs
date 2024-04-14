use sqlx::{Pool, Postgres};

use crate::models::{team::Team, player::Player};

pub async fn insert_team(team: Team, pool: &Pool<Postgres>) {
    sqlx::query!(
        "
    INSERT INTO team (id) 
    VALUES ($1)
    ",
        team.id,
    )
    .execute(pool)
    .await
    .unwrap();
}

pub async fn get_team_by_id(team_id: &str, pool: &Pool<Postgres>) -> Team {
    let team: Team = sqlx::query_as!(
        Team,
        "
        SELECT 
            t.id,
            ARRAY_AGG((p.id, p.name, t.id)) AS \"players!: Vec<Player>\"
        FROM team t
        INNER JOIN player p ON p.team_id = t.id
        WHERE p.team_id = $1
        GROUP BY t.id
        ",
        team_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    return team;
}
