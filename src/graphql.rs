use juniper::EmptySubscription;
use juniper::FieldResult;
use sea_orm::DatabaseConnection;
use crate::db::ToFieldError;
use crate::entity::user::{ self };
use crate::services::users_service::CreateUserInput;
use crate::services::UsersService;
#[derive(Clone)]
pub struct Context {
    pub db: DatabaseConnection,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn user(context: &Context, id: i32) -> FieldResult<Option<user::Model>> {
        UsersService::get_user_by_id(&context.db, id).await.map_err(|e| e.to_field_error())
    }
    async fn users(context: &Context) -> FieldResult<Vec<user::Model>> {
        UsersService::get_all_users(&context.db).await.map_err(|e| e.to_field_error())
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    async fn create_user(
        context: &Context,
        create_user_dto: CreateUserInput
    ) -> FieldResult<user::Model> {
        UsersService::create_user(&context.db, create_user_dto).await.map_err(|e| e.to_field_error())
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}
