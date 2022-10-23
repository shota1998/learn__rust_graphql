use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use actix_web::web::Data;
use actix_web::http::StatusCode;
use actix_web::middleware::{
    ErrorHandlers,
    ErrorHandlerResponse
};
use actix_web::dev::{
    ServiceResponse
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Enum, InputObject, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use log::info;
use once_cell::sync::Lazy;
use std::sync::Mutex;


#[derive(Clone, PartialEq)]
struct User {
    github_login: String,
    name: String,
    avatar: String,
    in_photos: Vec<String>
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


#[derive(Clone, PartialEq)]
struct Photo {
    id: usize,
    name: String,
    description: String,
    github_user: String,
    category: PhotoCategory,
    tagged_users: Vec<String>
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


#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum PhotoCategory {
    Selfie,
    Portrait,
    Action,
    Landscape,
    Graphic,
}

impl Default for PhotoCategory {
    fn default() -> Self {
        PhotoCategory::Portrait
    }
}


static SEQUENCE_ID: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

static USERS: Lazy<Mutex<Vec<User>>> = Lazy::new(|| Mutex::new(vec![
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

static PHOTOS: Lazy<Mutex<Vec<Photo>>> = Lazy::new(|| Mutex::new(vec![
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


struct Query;

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


struct Mutation;

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


type ApiSchema = Schema<Query, Mutation, EmptySubscription>;



async fn index(schema: web::Data<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    info!("log: index_playground");
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint(""));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))

}

fn not_found<B>(res: ServiceResponse<B>) 
-> actix_web::Result<ErrorHandlerResponse<B>> {
    info!("NOT_FOUND");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(
            HttpResponse::NotFound().finish().map_into_right_body()
        )
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let schema = Schema::build(Query, Mutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        let error_handlers = ErrorHandlers::new()
            .handler(StatusCode::NOT_FOUND, not_found);

        App::new()
            .app_data(Data::new(schema.clone()))
            .wrap(error_handlers)
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
            .service(web::resource("/").guard(guard::Post()).to(index))
            // .configure(configure_srervice)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
