use crate::Repository;
use eternize_models::section::Section;
use worker::D1Database;
use worker::Result as D1Result;

pub struct SectionD1Repositiry<'a> {
    db: &'a D1Database,
}

impl<'a> Repository for SectionD1Repositiry<'a> {
    type DB = &'a D1Database;
    type Entity = Section;

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
