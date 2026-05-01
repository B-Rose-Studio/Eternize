use crate::DB;
use worker::{D1Database, Env};

impl DB<D1Database> {
    pub fn new(env: &Env, name: impl Into<String>) -> Option<Self> {
        match env.d1(&name.into()) {
            Ok(db) => Some(Self { connection: db }),
            Err(_) => None,
        }
    }
}
