use crate::Repository;
use eternize_models::customize_page::CustomizePage;
use worker::D1Database;
use worker::Result as D1Result;

pub struct PageD1Repositiry<'a> {
    db: &'a D1Database,
}

impl<'a> Repository for PageD1Repositiry<'a> {
    type DB = &'a D1Database;
    type Entity = CustomizePage;

    fn new(db: Self::DB) -> Self {
        Self { db }
    }

    async fn save(&self, entity: Self::Entity) -> D1Result<Self::Entity> {
        todo!()
    }

    async fn read(&self, method: crate::ReadMethod) -> D1Result<Vec<Self::Entity>> {
        todo!()
    }

    async fn update(&self, entity: Self::Entity) -> D1Result<Self::Entity> {
        todo!()
    }

    async fn delete(&self, entity: Self::Entity) -> D1Result<()> {
        todo!()
    }
}
