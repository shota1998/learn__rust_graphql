pub mod graphql;
use std::sync::Arc;

use actix_web::{web, HttpResponse, dev::ServiceResponse, middleware::ErrorHandlerResponse};
use async_graphql::{
    Schema,
    EmptySubscription,
    http::{GraphQLPlaygroundConfig, playground_source}
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use log::info;

use crate::graphql::{
    query::Query,
    mutation::Mutation,
    types::AppSchema,
    database::connection::PgPool
};


pub fn configure_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::post().to(index))
            .route(web::get().to(index_playground)),
    );
}

async fn index(
    schema: web::Data<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let query = req.into_inner();
    schema.execute(query).await.into()
}

async fn index_playground() -> HttpResponse {
    info!("log: index_playground");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}

pub fn not_found<B>(res: ServiceResponse<B>) 
-> actix_web::Result<ErrorHandlerResponse<B>> {
    info!("NOT_FOUND");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(
            HttpResponse::NotFound().finish().map_into_right_body()
        )
    ))
}

pub fn create_schema_with_context(pool: PgPool) -> Schema<Query, Mutation, EmptySubscription> {
    let arc_pool = Arc::new(pool);

    Schema::build(Query, Mutation, EmptySubscription)
        .data(arc_pool)
        .enable_subscription_in_federation()
        .finish()
}