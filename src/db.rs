use dotenvy::dotenv;
use juniper::{graphql_value, FieldError};
use sea_orm::{ Database, DatabaseConnection, DbErr };

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Database::connect(database_url).await
}

pub trait ToFieldError {
    fn to_field_error(self) -> FieldError;
}

impl ToFieldError for DbErr {
    fn to_field_error(self) -> FieldError {
        FieldError::new(
            "Database error",
            graphql_value!({
                "internal_error": self.to_string()
            })
        )
    }
}