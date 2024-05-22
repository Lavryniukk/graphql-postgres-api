use chrono::Utc;
use juniper::graphql_value;
use juniper::EmptySubscription;
use juniper::FieldError;
use juniper::FieldResult;
use sea_orm::ActiveValue::NotSet;
use sea_orm::Set;
use sea_orm::{ DatabaseConnection, EntityTrait };
use crate::entity::user::{ self, Entity as User };
use sea_orm::entity::prelude::*;
#[derive(Clone)]
pub struct Context {
    pub db: DatabaseConnection,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn user(context: &Context, id: i32) -> Option<user::Model> {
        User::find_by_id(id).one(&context.db).await.expect("Error finding user")
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn create_user(
        context: &Context,
        name: String,
        email: String,
        password: String
    ) -> FieldResult<user::Model> {
        let existing_user = User::find()
            .filter(user::Column::Email.contains(&email))
            .one(&context.db).await
            .expect("Error finding user");
        if let Some(_) = existing_user {
            return Err(
                FieldError::new("User already exists", graphql_value!({ "email": email.clone() }))
            );
        }

        let active_user = user::ActiveModel {
            id: NotSet,
            name: Set(name),
            email: Set(email),
            role: Set("user".to_string()),
            password: Set(password),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            last_signed_in_at: Set(Utc::now().naive_utc()),
        };

        active_user
            .insert(&context.db).await
            .map_err(|e| {
                FieldError::new("Error creating user", graphql_value!({ "error": e.to_string() }))
            })
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
