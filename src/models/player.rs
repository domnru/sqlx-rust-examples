use sqlx::FromRow;

#[derive(Debug, FromRow, sqlx::Type)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub team_id: String,
}