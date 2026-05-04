use std::str::FromStr;

use short_uuid::ShortUuid;
use uuid::Uuid;

pub fn uuid_from_short_uuid(uuid: &str) -> Option<Uuid> {
    let short_uuid = ShortUuid::parse_str(uuid).ok()?;
    Some(short_uuid.to_uuid())
}

pub fn uuid_from_str(uuid: &str) -> Option<Uuid> {
    uuid_from_short_uuid(uuid).or_else(|| Uuid::from_str(uuid).ok())
}
