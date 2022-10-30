use actix_web::{web, App, HttpServer};
use actix_web::http::StatusCode;
use actix_web::middleware::ErrorHandlers;
use dotenv::dotenv;

mod graphql;
use crate::graphql::database::connection::create_connection_pool;
use ::graphql::{create_schema_with_context, configure_service, not_found};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Playground: http://localhost:8000");
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    dotenv().ok();
    
    let pool   = create_connection_pool();
    let schema = web::Data::new(create_schema_with_context(pool));

    HttpServer::new(move || {
        let error_handlers = ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found);

        App::new()
            .configure(configure_service)
            .app_data(schema.clone())
            .wrap(error_handlers)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
