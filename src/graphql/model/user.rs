use async_graphql::Object;
use crate::graphql::model::photo::Photo;
use crate::graphql::database::data::*;

#[derive(Clone, PartialEq)]
pub struct User {
    pub github_login: String,
    pub name: String,
    pub avatar: String,
    pub in_photos: Vec<String>
}

#[Object]
impl User {
    async fn github_login(&self) -> String {
        self.github_login.clone()
    }

    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn avatar(&self) -> String {
        self.avatar.clone()
    }

    async fn in_photos(&self) -> Vec<Photo> {
        let photos = PHOTOS.lock().unwrap().clone().into_iter()
            .filter(|photo| self.in_photos.contains(&photo.name)).collect();
        photos
    }

    async fn posted_photos(&self) -> Vec<Photo> {
        let photos = PHOTOS.lock().unwrap().clone().into_iter()
            .filter(|x| x.github_user == self.github_login).collect();
        photos
    }
}