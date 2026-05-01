pub mod d1;

use std::ops::Deref;

pub enum ListMethod {
    All,
    Page { numbers: usize, page: usize },
}

pub struct DB<T> {
    connection: T,
}

trait Repository {
    type DB;
    type Entity;

    fn new(db: Self::DB) -> Self;

    async fn save(&self, entity: Self::Entity) -> Result<Self::Entity, ()>;
    async fn list(&self, method: ListMethod) -> Result<Vec<Self::Entity>, ()>;
    async fn update(&self, entity: Self::Entity) -> Result<Self::Entity, ()>;
    async fn delete(&self, entity: Self::Entity) -> Result<(), ()>;
}

impl<T> Deref for DB<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}
