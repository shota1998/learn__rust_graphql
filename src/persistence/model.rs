use diesel::prelude::*;

#[derive(Identifiable, Queryable)]
#[diesel(table_name = articles)]
pub struct ArticleEntity {
    pub id: i32,
    pub title: String,
    pub body: String,
}
