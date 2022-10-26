use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::graphql::model::{photo::Photo, user::User};
use crate::graphql::enums::photo_category::PhotoCategory;

pub static SEQUENCE_ID: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

pub static USERS: Lazy<Mutex<Vec<User>>> = Lazy::new(|| Mutex::new(vec![
    User {
        github_login: "mHattrup".to_string(),
        name: "Mike Hattrup".to_string(),
        avatar: "".to_string(),
        in_photos: vec![]
    },
    User {
        github_login: "gPlake".to_string(),
        name: "Glen Plake".to_string(),
        avatar: "".to_string(),
        in_photos: vec!["Dropping the Heart Chute".to_string()]
    },
    User {
        github_login: "sSchmidt".to_string(),
        name: "Scot Schmidt".to_string(),
        avatar: "".to_string(),
        in_photos: ["Enjoying the sunshine", "25 laps on gunbarrel today"].iter().map(|&s| s.into()).collect()
    },
]));

pub static PHOTOS: Lazy<Mutex<Vec<Photo>>> = Lazy::new(|| Mutex::new(vec![
    Photo {
        id: 5,
        name: "Dropping the Heart Chute".to_string(),
        description: "The heart chute is one of my favorite chutes".to_string(),
        category: PhotoCategory::Action,
        github_user: "gPlake".to_string(),
        tagged_users: vec![]
    },
    Photo {
        id: 2,
        name: "Enjoying the sunshine".to_string(),
        description: "".to_string(),
        category: PhotoCategory::Selfie,
        github_user: "sSchmidt".to_string(),
        tagged_users: vec!["Mike Hattrup".to_string()]
    },
    Photo {
        id: 3,
        name: "Gunbarrel 25".to_string(),
        description: "25 laps on gunbarrel today".to_string(),
        category: PhotoCategory::Landscape,
        github_user: "sSchmidt".to_string(),
        tagged_users: vec!["Glen Plake", "Scot Schmidt"].iter().map(|&s| s.into()).collect()
    },
]));