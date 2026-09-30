use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};
use std::env;

pub async fn connect() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await?;

    db.execute_unprepared("PRAGMA journal_mode=WAL;").await?;
    Migrator::up(&db, None).await?;

    Ok(db)
}