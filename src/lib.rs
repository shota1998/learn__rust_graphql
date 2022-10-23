// pub fn configure_service(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/")
//             .route(web::post().to(index))
//             .route(
//                 web::get()
//                     .guard(guard::Header("upgrade", "websocket"))
//                     .to(index_ws),
//             )
//             .route(web::get().to(index_playground)),
//     );
// }

// async fn index(
//     schema: web::Data<AppSchema>,
//     http_req: HttpRequest,
//     req: GraphQLRequest,
// ) -> GraphQLResponse {
//     let mut query = req.into_inner();
//     let getting_role_result = common_utils::get_role(http_req);
//     query = query.data(getting_role_result);
//     schema.execute(query).await.into()
// }