use std::collections::HashMap;

use crate::ReadMethod;
use crate::Repository;
use crate::SectionRepository;
use eternize_models::section::Section;
use serde::Deserialize;
use uuid::Uuid;
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
        let query = "INSERT INTO sections (id, name, \"order\", page_id) VALUES (?, ?, ?, ?)";
        let statement = self.db.prepare(query).bind(&[
            entity.id.to_string().into(),
            entity.name.clone().into(),
            entity.order.into(),
            entity.page_id.to_string().into(),
        ])?;
        statement.run().await?;
        Ok(entity)
    }

    async fn read(&self, method: ReadMethod) -> D1Result<Vec<Self::Entity>> {
        match method {
            ReadMethod::All => {
                let statement = self.db.prepare("SELECT * FROM sections");
                statement.all().await?.results::<Self::Entity>()
            }
            ReadMethod::ById(id) => {
                let statement = self
                    .db
                    .prepare("SELECT * FROM sections WHERE id = ?")
                    .bind(&[id.to_string().into()])?;
                statement.all().await?.results::<Self::Entity>()
            }
            ReadMethod::Page { numbers, page } => {
                let offset = (page.saturating_sub(1)) * numbers;
                let statement = self
                    .db
                    .prepare("SELECT * FROM sections LIMIT ? OFFSET ?")
                    .bind(&[(numbers as u32).into(), (offset as u32).into()])?;
                statement.all().await?.results::<Self::Entity>()
            }
        }
    }

    async fn update(&self, entity: Self::Entity) -> D1Result<Self::Entity> {
        let query = "UPDATE sections SET name = ?, \"order\" = ?, page_id = ? WHERE id = ?";
        let statement = self.db.prepare(query).bind(&[
            entity.name.clone().into(),
            entity.order.into(),
            entity.page_id.to_string().into(),
            entity.id.to_string().into(),
        ])?;
        statement.run().await?;
        Ok(entity)
    }

    async fn delete(&self, entity: Self::Entity) -> D1Result<()> {
        let statement = self
            .db
            .prepare("DELETE FROM sections WHERE id = ?")
            .bind(&[entity.id.to_string().into()])?;
        statement.run().await?;
        Ok(())
    }
}

impl<'a> SectionRepository for SectionD1Repositiry<'a> {
    async fn add_property(&self, section_id: Uuid, name: String, value: String) -> D1Result<Uuid> {
        let prop_id = Uuid::new_v4();
        let query = "INSERT INTO propertys (id, name, value, section_id) VALUES (?, ?, ?, ?)";

        let statement = self.db.prepare(query).bind(&[
            prop_id.to_string().into(),
            name.into(),
            value.into(),
            section_id.to_string().into(),
        ])?;
        statement.run().await?;

        Ok(prop_id)
    }

    async fn add_properties(
        &self,
        section_id: Uuid,
        properties: HashMap<String, String>,
    ) -> D1Result<()> {
        if properties.is_empty() {
            return Ok(());
        }

        let mut query = String::from("INSERT INTO propertys (id, name, value, section_id) VALUES ");
        let mut bindings = Vec::new();
        let mut placeholders = Vec::new();

        for (name, value) in properties {
            placeholders.push("(?, ?, ?, ?)");
            bindings.push(worker::wasm_bindgen::JsValue::from_str(
                &Uuid::new_v4().to_string(),
            ));

            bindings.push(worker::wasm_bindgen::JsValue::from_str(&name));
            bindings.push(worker::wasm_bindgen::JsValue::from_str(&value));
            bindings.push(worker::wasm_bindgen::JsValue::from_str(
                &section_id.to_string(),
            ));
        }

        query.push_str(&placeholders.join(", "));

        let statement = self.db.prepare(&query).bind(bindings.as_slice())?;
        statement.run().await?;

        Ok(())
    }

    async fn get_properties(&self, section_id: Uuid) -> D1Result<HashMap<String, String>> {
        let query = "SELECT name, value FROM propertys WHERE section_id = ?";
        let statement = self
            .db
            .prepare(query)
            .bind(&[section_id.to_string().into()])?;

        #[derive(Deserialize)]
        struct RawProperty {
            name: String,
            value: Option<String>,
        }

        let raw_properties: Vec<RawProperty> = statement.all().await?.results()?;
        let mut props_map = HashMap::new();

        for prop in raw_properties {
            props_map.insert(prop.name, prop.value.unwrap_or_default());
        }

        Ok(props_map)
    }

    async fn update_property(&self, prop_id: Uuid, name: String, value: String) -> D1Result<()> {
        let query = "UPDATE propertys SET name = ?, value = ? WHERE id = ?";

        let statement = self.db.prepare(query).bind(&[
            name.into(),
            value.into(),
            prop_id.to_string().into(),
        ])?;
        statement.run().await?;

        Ok(())
    }

    async fn delete_property(&self, prop_id: Uuid) -> D1Result<()> {
        let query = "DELETE FROM propertys WHERE id = ?";
        let statement = self.db.prepare(query).bind(&[prop_id.to_string().into()])?;
        statement.run().await?;

        Ok(())
    }
}
