use crate::ReadMethod;
use crate::Repository;
use crate::SignatureRepository;
use eternize_models::signature::Signature;
use worker::D1Database;
use worker::Result as D1Result;

pub struct SignatureD1Repositiry<'a> {
    db: &'a D1Database,
}

impl<'a> Repository for SignatureD1Repositiry<'a> {
    type DB = &'a D1Database;
    type Entity = Signature;

    fn new(db: Self::DB) -> Self {
        Self { db }
    }

    async fn save(&self, entity: Self::Entity) -> D1Result<Self::Entity> {
        let query = "INSERT INTO signatures (id, name, price, max_sections) VALUES (?, ?, ?, ?)";
        let statement = self.db.prepare(query).bind(&[
            entity.id.to_string().into(),
            entity.name.clone().into(),
            entity.price.into(),
            entity.max_sections.into(),
        ])?;
        statement.run().await?;
        Ok(entity)
    }

    async fn read(&self, method: ReadMethod) -> D1Result<Vec<Self::Entity>> {
        match method {
            ReadMethod::All => {
                let statement = self.db.prepare("SELECT * FROM signatures");
                statement.all().await?.results::<Self::Entity>()
            }
            ReadMethod::ById(id) => {
                let statement = self
                    .db
                    .prepare("SELECT * FROM signatures WHERE id = ?")
                    .bind(&[id.to_string().into()])?;
                statement.all().await?.results::<Self::Entity>()
            }
            ReadMethod::Page { numbers, page } => {
                let offset = (page.saturating_sub(1)) * numbers;
                let statement = self
                    .db
                    .prepare("SELECT * FROM signatures LIMIT ? OFFSET ?")
                    .bind(&[(numbers as u32).into(), (offset as u32).into()])?;
                statement.all().await?.results::<Self::Entity>()
            }
        }
    }

    async fn update(&self, entity: Self::Entity) -> D1Result<Self::Entity> {
        let query = "UPDATE signatures SET name = ?, price = ?, max_sections = ? WHERE id = ?";
        let statement = self.db.prepare(query).bind(&[
            entity.name.clone().into(),
            entity.price.into(),
            entity.max_sections.into(),
            entity.id.to_string().into(),
        ])?;
        statement.run().await?;
        Ok(entity)
    }

    async fn delete(&self, entity: Self::Entity) -> D1Result<()> {
        let statement = self
            .db
            .prepare("DELETE FROM signatures WHERE id = ?")
            .bind(&[entity.id.to_string().into()])?;
        statement.run().await?;
        Ok(())
    }
}

impl<'a> SignatureRepository for SignatureD1Repositiry<'a> {}
