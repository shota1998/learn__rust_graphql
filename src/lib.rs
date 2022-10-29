pub mod graphql;
use std::sync::Arc;

use async_graphql::dataloader::DataLoader;
use async_graphql::{Schema, EmptySubscription};
use crate::graphql::{query::Query, mutation::Mutation};

use crate::graphql::database::connection::PgPool;

pub fn create_schema_with_context(pool: PgPool) -> Schema<Query, Mutation, EmptySubscription> {
    let arc_pool    = Arc::new(pool);
    let cloned_pool = Arc::clone(&arc_pool);
    // let details_data_loader =
    //     DataLoader::new(DetailsLoader { pool: cloned_pool }, actix_rt::spawn).max_batch_size(10);

    Schema::build(Query, Mutation, EmptySubscription)
        .data(arc_pool)
        // .data(details_data_loader)
        .enable_subscription_in_federation()
        .finish()
}