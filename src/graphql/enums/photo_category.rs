use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PhotoCategory {
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