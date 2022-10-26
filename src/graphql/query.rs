use async_graphql::Object;
use log::info;
use super::model::{user::User, photo::Photo};
use super::database::data::*;

pub struct Query;

#[Object]
impl Query {
    async fn total_photos(&self) -> usize {
        info!("query: total_photos");
        PHOTOS.lock().unwrap().len()
    }

    async fn all_photos(&self) -> Vec<Photo> {
        info!("query: all_photos");
        PHOTOS.lock().unwrap().clone()
    }

    async fn all_users(&self) -> Vec<User> {
        info!("query: all_users");
        USERS.lock().unwrap().clone()
    }
}