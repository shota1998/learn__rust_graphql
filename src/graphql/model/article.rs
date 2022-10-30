use async_graphql::Object;

#[derive(Clone, PartialEq)]
pub struct Article {
    pub id: usize,
    pub title: String,
    pub body: String
}

#[Object]
impl Article {
    async fn id(&self) -> usize {
        self.id
    }

    async fn title(&self) -> String {
        self.title.clone()
    }

    async fn body(&self) -> String {
        self.body.clone()
    }
}