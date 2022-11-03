use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Entity,
    Comment,
    Reply,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct EntityId(String);


impl EntityId {
    pub fn new() -> EntityId {
        let id = Uuid::new_v4().to_string();
        return EntityId(id);
    }

    pub fn from(id: String) -> Result<EntityId, String> {
        if id.len() > 0 {
            return Ok(EntityId(id));
        }

        return Err("invalid EntityId: empty id".to_string());
    }
}

impl AsRef<str> for EntityId {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}