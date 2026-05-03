use std::collections::HashMap;

use crate::PageRepository;
use crate::ReadMethod;
use crate::Repository;
use eternize_models::customize_page::CustomizePage;
use serde::Deserialize;
use uuid::Uuid;
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
        let query = "INSERT INTO customize_pages (id, name, title, purchased_in, renewed_in, active, user_id, signature_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";

        let active_int = if entity.active { 1 } else { 0 };

        let statement = self.db.prepare(query).bind(&[
            entity.id.to_string().into(),
            entity.name.clone().into(),
            entity.title.clone().into(),
            entity.purchased_in.to_rfc3339().into(),
            entity.renewed_in.to_rfc3339().into(),
            active_int.into(),
            entity.user_id.to_string().into(),
            entity.signature_id.to_string().into(),
        ])?;
        statement.run().await?;
        Ok(entity)
    }

    async fn read(&self, method: ReadMethod) -> D1Result<Vec<Self::Entity>> {
        match method {
            ReadMethod::All => {
                let statement = self.db.prepare("SELECT * FROM customize_pages");
                statement.all().await?.results::<Self::Entity>()
            }
            ReadMethod::ById(id) => {
                let statement = self
                    .db
                    .prepare("SELECT * FROM customize_pages WHERE id = ?")
                    .bind(&[id.to_string().into()])?;
                statement.all().await?.results::<Self::Entity>()
            }
            ReadMethod::Page { numbers, page } => {
                let offset = (page.saturating_sub(1)) * numbers;
                let statement = self
                    .db
                    .prepare("SELECT * FROM customize_pages LIMIT ? OFFSET ?")
                    .bind(&[(numbers as u32).into(), (offset as u32).into()])?;
                statement.all().await?.results::<Self::Entity>()
            }
        }
    }

    async fn update(&self, entity: Self::Entity) -> D1Result<Self::Entity> {
        let query = "UPDATE customize_pages SET name = ?, title = ?, purchased_in = ?, renewed_in = ?, active = ?, user_id = ?, signature_id = ? WHERE id = ?";

        let active_int = if entity.active { 1 } else { 0 };

        let statement = self.db.prepare(query).bind(&[
            entity.name.clone().into(),
            entity.title.clone().into(),
            entity.purchased_in.to_rfc3339().into(),
            entity.renewed_in.to_rfc3339().into(),
            active_int.into(),
            entity.user_id.to_string().into(),
            entity.signature_id.to_string().into(),
            entity.id.to_string().into(),
        ])?;
        statement.run().await?;
        Ok(entity)
    }

    async fn delete(&self, entity: Self::Entity) -> D1Result<()> {
        let statement = self
            .db
            .prepare("DELETE FROM customize_pages WHERE id = ?")
            .bind(&[entity.id.to_string().into()])?;
        statement.run().await?;
        Ok(())
    }
}

impl<'a> PageRepository for PageD1Repositiry<'a> {
    async fn get_all_properties(&self, page_id: Uuid) -> D1Result<HashMap<String, String>> {
        let query = "
                    SELECT p.name, p.value
                    FROM propertys p
                    INNER JOIN sections s ON p.section_id = s.id
                    WHERE s.page_id = ?
                ";

        let statement = self.db.prepare(query).bind(&[page_id.to_string().into()])?;

        #[derive(Deserialize)]
        struct RawProperty {
            name: String,
            value: Option<String>,
        }

        let raw_properties: Vec<RawProperty> = statement.all().await?.results()?;

        let mut page_props_map: HashMap<String, String> = HashMap::new();

        for prop in raw_properties {
            page_props_map.insert(prop.name, prop.value.unwrap_or_default());
        }

        Ok(page_props_map)
    }
}
