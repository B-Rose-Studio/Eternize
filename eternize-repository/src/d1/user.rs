use crate::ReadMethod;
use crate::Repository;
use crate::UserRepository;
use eternize_models::user::User;
use eternize_models::user::UserType;
use worker::D1Database;
use worker::Result as D1Result;

pub struct UserD1Repository<'a> {
    db: &'a D1Database,
}

impl<'a> Repository for UserD1Repository<'a> {
    type DB = &'a D1Database;
    type Entity = User;

    fn new(db: Self::DB) -> Self {
        Self { db }
    }

    async fn save(&self, entity: Self::Entity) -> D1Result<Self::Entity> {
        let query = "INSERT INTO users (id, type, email, password, first_name, last_name, phone, active, birthdate) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";

        let type_str = match entity.user_type {
            UserType::Admin => "Admin",
            UserType::Cleint => "Client",
        };

        let active_int = if entity.active { 1 } else { 0 };

        let statement = self.db.prepare(query).bind(&[
            entity.id.to_string().into(),
            type_str.into(),
            entity.email.clone().into(),
            entity.password.clone().into(),
            entity.first_name.clone().into(),
            entity.last_name.clone().into(),
            entity.phone.clone().into(),
            active_int.into(),
            entity.birthdate.to_rfc3339().into(),
        ])?;

        statement.run().await?;
        Ok(entity)
    }

    async fn read(&self, method: ReadMethod) -> D1Result<Vec<Self::Entity>> {
        match method {
            ReadMethod::All => {
                let query = "SELECT * FROM users";
                let statement = self.db.prepare(query);

                statement.all().await?.results()
            }

            ReadMethod::ById(id) => {
                let query = "SELECT * FROM users WHERE id = ?";
                let statement = self.db.prepare(query).bind(&[id.to_string().into()])?;
                statement.all().await?.results()
            }

            ReadMethod::Page { numbers, page } => {
                let offset = (page.saturating_sub(1)) * numbers;
                let query = "SELECT * FROM users LIMIT ? OFFSET ?";
                let statement = self
                    .db
                    .prepare(query)
                    .bind(&[(numbers as u32).into(), (offset as u32).into()])?;
                statement.all().await?.results()
            }
        }
    }

    async fn update(&self, entity: Self::Entity) -> D1Result<Self::Entity> {
        let query = "UPDATE users SET type = ?, email = ?, password = ?, first_name = ?, last_name = ?, phone = ?, active = ?, birthdate = ? WHERE id = ?";

        let type_str = match entity.user_type {
            UserType::Admin => "Admin",
            UserType::Cleint => "Client",
        };

        let active_int = if entity.active { 1 } else { 0 };

        let statement = self.db.prepare(query).bind(&[
            type_str.into(),
            entity.email.clone().into(),
            entity.password.clone().into(),
            entity.first_name.clone().into(),
            entity.last_name.clone().into(),
            entity.phone.clone().into(),
            active_int.into(),
            entity.birthdate.to_rfc3339().into(),
            entity.id.to_string().into(),
        ])?;

        statement.run().await?;
        Ok(entity)
    }

    async fn delete(&self, entity: Self::Entity) -> D1Result<()> {
        let query = "DELETE FROM users WHERE id = ?";
        let statement = self
            .db
            .prepare(query)
            .bind(&[entity.id.to_string().into()])?;
        statement.run().await?;

        Ok(())
    }
}

impl<'a> UserRepository for UserD1Repository<'a> {}
