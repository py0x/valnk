use std::fmt;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Submission,
    Comment,
    Reply,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct EntityId(String);


impl EntityId {
    pub fn new() -> EntityId {
        let id = Uuid::new_v4().to_string();
        return EntityId(id);
    }

    pub fn from(id: impl Into<String>) -> Result<EntityId, String> {
        let id_str = id.into();
        if id_str.len() > 0 {
            return Ok(EntityId(id_str));
        }

        return Err("invalid EntityId: empty id".to_string());
    }
}

impl AsRef<str> for EntityId {
    fn as_ref(&self) -> &str {
        return &self.0;
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}