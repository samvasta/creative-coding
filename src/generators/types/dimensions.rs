use std::collections::HashMap;

use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_json::Value;

pub trait FromJson {
    type ValueType;
    fn name(&self) -> String;
    fn json_type(&self) -> String;
    fn from_json(&self, value: &Value) -> Self::ValueType;
    fn get_default(&self) -> Self::ValueType;
}

#[derive(Serialize, Deserialize)]
pub struct BooleanInput {
    pub default: bool,
}

impl FromJson for BooleanInput {
    type ValueType = bool;
    fn name(&self) -> String {
        String::from("boolean")
    }
    fn json_type(&self) -> String {
        String::from("boolean")
    }
    fn from_json(&self, value: &Value) -> bool {
        match value {
            Value::Bool(_) => true,
            _ => self.default,
        }
    }
    fn get_default(&self) -> Self::ValueType {
        self.default
    }
}

#[derive(Serialize, Deserialize)]
pub struct IntegerInput {
    pub min: i64,
    pub max: i64,
    pub default: i64,
}

impl FromJson for IntegerInput {
    type ValueType = i64;
    fn name(&self) -> String {
        String::from("integer")
    }
    fn json_type(&self) -> String {
        String::from("number")
    }
    fn from_json(&self, value: &Value) -> i64 {
        match value {
            Value::Number(num) => match num.as_i64() {
                Some(int_value) => {
                    if int_value >= self.min && int_value <= self.max {
                        return int_value;
                    } else {
                        return self.default;
                    }
                }
                _ => self.default,
            },
            _ => self.default,
        }
    }
    fn get_default(&self) -> Self::ValueType {
        self.default
    }
}

#[derive(Serialize, Deserialize)]
pub struct FloatInput {
    pub min: f64,
    pub max: f64,
    pub default: f64,
}
impl FromJson for FloatInput {
    type ValueType = f64;
    fn name(&self) -> String {
        String::from("float")
    }
    fn json_type(&self) -> String {
        String::from("number")
    }
    fn from_json(&self, value: &Value) -> f64 {
        match value {
            Value::Number(num) => match num.as_f64() {
                Some(float_value) => {
                    if float_value >= self.min && float_value <= self.max {
                        return float_value;
                    } else {
                        return self.default;
                    }
                }
                _ => self.default,
            },
            _ => self.default,
        }
    }
    fn get_default(&self) -> Self::ValueType {
        self.default
    }
}

#[derive(Serialize, Deserialize)]
pub struct StringInput {
    pub default: String,
}

impl FromJson for StringInput {
    type ValueType = String;
    fn name(&self) -> String {
        String::from("string")
    }
    fn json_type(&self) -> String {
        String::from("string")
    }
    fn from_json(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            _ => self.default.clone(),
        }
    }
    fn get_default(&self) -> Self::ValueType {
        self.default.clone()
    }
}

#[derive(Serialize, Deserialize)]
pub struct OptionsInput {
    pub options: Vec<String>,
    pub default: String,
}

impl FromJson for OptionsInput {
    type ValueType = String;
    fn name(&self) -> String {
        String::from("options")
    }
    fn json_type(&self) -> String {
        String::from("string")
    }
    fn from_json(&self, value: &Value) -> String {
        match value {
            Value::String(s) => {
                if self.options.contains(&s) {
                    return s.clone();
                } else {
                    return self.default.clone();
                }
            }
            _ => self.default.clone(),
        }
    }
    fn get_default(&self) -> Self::ValueType {
        self.default.clone()
    }
}

#[derive(Deserialize)]
pub struct GeneratorDimensionInfo<T: FromJson + Serialize> {
    pub name: &'static str,
    pub description: &'static str,
    pub data_info: T,
}

impl<T: FromJson + Serialize> Serialize for GeneratorDimensionInfo<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("GeneratorDimensionInfo", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("data_info", &self.data_info)?;
        state.serialize_field("data_type", &(self.data_info.name()))?;
        state.serialize_field("json_type", &(self.data_info.json_type()))?;
        state.end()
    }
}

impl<T: FromJson + Serialize> GeneratorDimensionInfo<T> {
    pub fn get_value_from_json(&self, raw_json: HashMap<String, Value>) -> T::ValueType {
        match raw_json.get(self.name) {
            Some(value) => self.data_info.from_json(value),
            _ => self.data_info.get_default(),
        }
    }
}
