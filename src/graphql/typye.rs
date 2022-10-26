// use actix_web::web::Query;
use async_graphql::{Schema, EmptySubscription};
use super::query::Query;
use super::mutation::Mutation;

pub type ApiSchema = Schema<Query, Mutation, EmptySubscription>;