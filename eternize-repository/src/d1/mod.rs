use crate::DB;
use worker::{D1Database, Env};

mod page;
mod section;
mod signature;
mod user;

pub use page::*;
pub use section::*;
pub use signature::*;
pub use user::*;

impl DB<D1Database> {
    pub fn new(env: &Env, name: impl Into<String>) -> Option<Self> {
        match env.d1(&name.into()) {
            Ok(db) => Some(Self { connection: db }),
            Err(_) => None,
        }
    }
}
