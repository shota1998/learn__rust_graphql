use actix_web::http::StatusCode;
use actix_web::{
    App,
    HttpServer,
    HttpResponse,
    web
};
use actix_web::middleware::{
    ErrorHandlers,
    ErrorHandlerResponse
};
use actix_web::dev::{
    ServiceResponse
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation,
    EmptySubscription,
    Request,
    Response,
    Schema,
};
use log::info;

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

// async fn graphql_handler(schema: Extension<BlogSchema>, req: Json<Request>) -> Json<Response> {
//     schema.execute(req.0).await.into()
// }

fn not_found<B>(res: ServiceResponse<B>) 
-> actix_web::Result<ErrorHandlerResponse<B>> {
    info!("NOT_FOUND");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(
            HttpResponse::NotFound().finish().map_into_right_body()
        )
    ))
}

pub async fn index() -> HttpResponse {
    info!("info");
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        let error_handlers = ErrorHandlers::new()
            .handler(StatusCode::NOT_FOUND, not_found);

        // let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        //     .finish();

        App::new()
            .wrap(error_handlers)
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
