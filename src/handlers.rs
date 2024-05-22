use actix_web::{ get, route, web, Error, HttpResponse, Responder };
use actix_web_lab::respond::Html;
use juniper::http::{ graphiql::graphiql_source, GraphQLRequest };

use crate::graphql::{ create_schema, Context, Schema };

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
    connection: web::Data<Context>,
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        db: connection.db.clone(),
    };

    let res = data.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

pub fn register(config: &mut web::ServiceConfig) {
    config.app_data(web::Data::new(create_schema())).service(graphql).service(graphql_playground);
}
