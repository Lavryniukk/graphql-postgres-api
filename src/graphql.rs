use juniper::graphql_value;
use juniper::EmptySubscription;
use juniper::FieldError;
use juniper::FieldResult;
use sea_orm::DatabaseConnection;
use crate::entity::user::{ self };
use crate::services::UsersService;
#[derive(Clone)]
pub struct Context {
    pub db: DatabaseConnection,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn user(context: &Context, id: i32) -> Option<user::Model> {
        UsersService::get_user_by_id(&context.db, id).await.unwrap()
    }
    async fn users(context: &Context) -> Vec<user::Model> {
        UsersService::get_all_users(&context.db).await.unwrap()
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
        UsersService::create_user(&context.db, name, email, password).await.map_err(|e|
            FieldError::new(e, graphql_value!({ "internal_error": "An error occurred" }))
        )
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
