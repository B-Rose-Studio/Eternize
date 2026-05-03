pub mod d1;

use eternize_models::{
    customize_page::CustomizePage, section::Section, signature::Signature, user::User,
};
use std::{collections::HashMap, ops::Deref};
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

impl<T> Deref for DB<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}

impl<T> DB<T> {
    pub fn as_ref(&self) -> &T {
        &self.connection
    }
}

#[allow(async_fn_in_trait)]
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

#[allow(async_fn_in_trait)]
pub trait PageRepository: Repository<Entity = CustomizePage> {
    async fn get_all_properties(&self, page_id: Uuid) -> WorkerResult<HashMap<String, String>>;
    async fn get_sections(&self, page_id: Uuid) -> WorkerResult<Vec<Section>>;
}

#[allow(async_fn_in_trait)]
pub trait SectionRepository: Repository<Entity = Section> {
    async fn add_property(
        &self,
        section_id: Uuid,
        name: String,
        value: String,
    ) -> WorkerResult<Uuid>;

    async fn get_properties(&self, section_id: Uuid) -> WorkerResult<HashMap<String, String>>;

    async fn add_properties(
        &self,
        section_id: Uuid,
        properties: HashMap<String, String>,
    ) -> WorkerResult<()>;

    async fn update_property(&self, prop_id: Uuid, name: String, value: String)
    -> WorkerResult<()>;

    async fn delete_property(&self, prop_id: Uuid) -> WorkerResult<()>;
}
