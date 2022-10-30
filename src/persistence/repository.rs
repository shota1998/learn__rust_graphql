use diesel::prelude::*;

use crate::persistence::model::ArticleEntity;
use crate::persistence::schema::{articles};

pub fn get_all(conn: &mut PgConnection) -> QueryResult<Vec<ArticleEntity>> {
    use crate::persistence::schema::planets::dsl::*;

    planets.load(conn)
}
