pub mod d1;

use eternize_models::{
    customize_page::CustomizePage, section::Section, signature::Signature, user::User,
};
use std::ops::Deref;
use uuid::Uuid;
use worker::Result as WorkerResult;

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

    async fn save(&self, entity: Self::Entity) -> WorkerResult<Self::Entity>;
    async fn read(&self, method: ReadMethod) -> WorkerResult<Vec<Self::Entity>>;
    async fn update(&self, entity: Self::Entity) -> WorkerResult<Self::Entity>;
    async fn delete(&self, entity: Self::Entity) -> WorkerResult<()>;
}

pub trait UserRepository: Repository<Entity = User> {}
pub trait SignatureRepository: Repository<Entity = Signature> {}
pub trait SectionRepository: Repository<Entity = Section> {}
pub trait PageRepository: Repository<Entity = CustomizePage> {}

impl<T> Deref for DB<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}
