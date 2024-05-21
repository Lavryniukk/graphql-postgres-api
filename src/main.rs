use sea_orm::{Database, DatabaseConnection, DbErr};

fn main() {
    println!("Hello, world!");
}


async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    Database::connect("protocol://username:password@host/database").await
}