use async_graphql::Object;
use crate::graphql::model::user::User;
use crate::graphql::database::data::*;
use crate::graphql::enums::photo_category::PhotoCategory;

#[derive(Clone, PartialEq)]
pub struct Photo {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub github_user: String,
    pub category: PhotoCategory,
    pub tagged_users: Vec<String>
}

#[Object]
impl Photo {
    async fn id(&self) -> usize {
        self.id
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn description(&self) -> String {
        self.description.clone()
    }

    async fn category(&self) -> PhotoCategory {
        self.category
    }

    async fn tagged_users(&self) -> Vec<User> {
        let users = USERS.lock().unwrap().clone().into_iter()
            .filter(|user| self.tagged_users.contains(&user.name)).collect();
        users
    }

    async fn posted_by(&self) -> User {
        let user = USERS.lock().unwrap().clone().into_iter()
            .find(|user| user.github_login == self.github_user).unwrap();
        user
    }
}