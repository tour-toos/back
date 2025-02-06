use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};


pub async fn connect(options: ConnectOptions) -> Result<(), DbErr> {
    let db: DatabaseConnection = Database::connect(options).await?;
    Ok(())
}
