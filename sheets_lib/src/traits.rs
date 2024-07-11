use anyhow::Result;
use serde_json::{Map, Value};

use super::Error;

pub trait ConvertValue {
    fn to_obj(&self) -> Result<&Map<String, Value>>;
    fn to_array(&self) -> Result<&Vec<Value>>;
    fn to_str(&self) -> Result<&str>;
}

impl ConvertValue for Value {
    fn to_obj(&self) -> Result<&Map<String, Value>> {
        self.as_object().ok_or(Error::ExpectObject.into())
    }

    fn to_array(&self) -> Result<&Vec<Value>> {
        self.as_array().ok_or(Error::ExpectObject.into())
    }

    fn to_str(&self) -> Result<&str> {
        self.as_str().ok_or(Error::ExpectString.into())
    }
}

pub trait GetField {
    fn get_field(&self, key: &str) -> Result<&Value>;
}

impl GetField for Map<String, Value> {
    fn get_field(&self, key: &str) -> Result<&Value> {
        self.get(key)
            .ok_or(Error::FieldNotFound(key.to_string()).into())
    }
}
