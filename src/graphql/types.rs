use async_graphql::{Schema, EmptySubscription};
use super::query::Query;
use super::mutation::Mutation;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;