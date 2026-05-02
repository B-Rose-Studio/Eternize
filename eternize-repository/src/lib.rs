pub mod d1;

use std::ops::Deref;
use uuid::Uuid;
use worker::Result as D1Result;

pub enum ReadMethod {
    All,
    Page { numbers: usize, page: usize },
    ById(Uuid),
}

pub struct DB<T> {
    connection: T,
}

pub trait Repository {
    type DB;
    type Entity;

    fn new(db: Self::DB) -> Self;

    async fn save(&self, entity: Self::Entity) -> D1Result<Self::Entity>;
    async fn read(&self, method: ReadMethod) -> D1Result<Vec<Self::Entity>>;
    async fn update(&self, entity: Self::Entity) -> D1Result<Self::Entity>;
    async fn delete(&self, entity: Self::Entity) -> D1Result<()>;
}

impl<T> Deref for DB<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}
