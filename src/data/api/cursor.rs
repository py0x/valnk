use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
use std::collections::HashMap;
use std::convert::TryFrom;

use serde::{Serialize, Deserialize};

use aws_sdk_dynamodb::model::AttributeValue;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Cursor(HashMap<String, String>);

impl TryFrom<HashMap<String, AttributeValue>> for Cursor {
    type Error = serde_dynamo::Error;

    fn try_from(value: HashMap<String, AttributeValue>) -> std::result::Result<Self, Self::Error> {
        serde_dynamo::from_item(value)
    }
}

impl TryFrom<Cursor> for HashMap<String, AttributeValue> {
    type Error = serde_dynamo::Error;

    fn try_from(value: Cursor) -> std::result::Result<Self, Self::Error> {
        serde_dynamo::to_item(value.0)
    }
}


impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = serde_json::to_string(&self.0).map_err(|e| fmt::Error)?;

        write!(f, "{}", s)
    }
}

impl FromStr for Cursor {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}