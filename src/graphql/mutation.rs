use async_graphql::{InputObject, Object};
use log::info;
use crate::graphql::database::data::{SEQUENCE_ID, PHOTOS};
use super::{
    enums::photo_category::PhotoCategory, 
    model::photo::Photo
};

pub struct Mutation;

#[derive(InputObject)]
struct PostPhotoInput {
    name: String,
    description: String,
    github_user: String,
    #[graphql(default_with = "PhotoCategory::default()")]
    category: PhotoCategory,
}

#[Object]
impl Mutation {
    async fn post_photo(&self, input: PostPhotoInput) -> Photo {
        let mut id = SEQUENCE_ID.lock().unwrap();
        *id += 1;
        let photo = Photo {
            id: *id, 
            name: input.name, 
            description: input.description,
            github_user: input.github_user,
            category: input.category,
            tagged_users: vec![]
        };
        PHOTOS.lock().unwrap().push(photo.clone());
        info!("mutation: post_photo");
        photo
    }
}