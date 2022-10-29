use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use actix_web::web::Data;
use actix_web::http::StatusCode;
use actix_web::middleware::{
    ErrorHandlers,
    ErrorHandlerResponse
};
use actix_web::dev::{
    ServiceResponse
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use log::info;

mod graphql;
use crate::graphql::{typye::ApiSchema, mutation::Mutation, query::Query};
use crate::graphql::database::connection::create_connection_pool;

use ::graphql::create_schema_with_context;


async fn index(schema: web::Data<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    info!("log: index_playground");
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint(""));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))

}

fn not_found<B>(res: ServiceResponse<B>) 
-> actix_web::Result<ErrorHandlerResponse<B>> {
    info!("NOT_FOUND");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(
            HttpResponse::NotFound().finish().map_into_right_body()
        )
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let pool   = create_connection_pool();
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();
    let schema = web::Data::new(create_schema_with_context(pool));

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        let error_handlers = ErrorHandlers::new()
            .handler(StatusCode::NOT_FOUND, not_found);

        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(error_handlers)
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
            .service(web::resource("/").guard(guard::Post()).to(index))
            // .configure(configure_srervice)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
