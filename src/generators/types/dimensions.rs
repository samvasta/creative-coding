use std::{collections::HashMap, fmt};

use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_json::{json, Value};

pub trait InputInfo {
    fn name(&self) -> String;
    fn json_type(&self) -> String;
    fn to_json(&self) -> Value;
    fn clone_box(&self) -> Box<dyn InputInfo>;
}

pub trait InputExtractor<T>: InputInfo {
    fn parse_json(&self, value: &Value) -> T;
    fn get_default(&self) -> T;

    fn from_dimensions(&self, key: &'static str, raw_dimensions_json: HashMap<String, Value>) -> T {
        match raw_dimensions_json.get(&self.name()) {
            Some(value) => self.parse_json(value),
            _ => self.get_default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BooleanInput {
    pub default: bool,
}

impl InputInfo for BooleanInput {
    fn name(&self) -> String {
        String::from("boolean")
    }
    fn json_type(&self) -> String {
        String::from("boolean")
    }
    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
    fn clone_box(&self) -> Box<dyn InputInfo> {
        Box::new(self.clone())
    }
}
impl InputExtractor<bool> for BooleanInput {
    fn parse_json(&self, value: &Value) -> bool {
        match value {
            Value::Bool(_) => true,
            _ => self.default,
        }
    }
    fn get_default(&self) -> bool {
        self.default
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IntegerInput {
    pub min: i64,
    pub max: i64,
    pub default: i64,
}

impl InputInfo for IntegerInput {
    fn name(&self) -> String {
        String::from("integer")
    }
    fn json_type(&self) -> String {
        String::from("number")
    }
    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
    fn clone_box(&self) -> Box<dyn InputInfo> {
        Box::new(self.clone())
    }
}
impl InputExtractor<i64> for IntegerInput {
    fn parse_json(&self, value: &Value) -> i64 {
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
    fn get_default(&self) -> i64 {
        self.default
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FloatInput {
    pub min: f64,
    pub max: f64,
    pub default: f64,
}
impl InputInfo for FloatInput {
    fn name(&self) -> String {
        String::from("float")
    }
    fn json_type(&self) -> String {
        String::from("number")
    }
    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
    fn clone_box(&self) -> Box<dyn InputInfo> {
        Box::new(self.clone())
    }
}
impl InputExtractor<f64> for FloatInput {
    fn parse_json(&self, value: &Value) -> f64 {
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
    fn get_default(&self) -> f64 {
        self.default
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StringInput {
    pub default: String,
}

impl InputInfo for StringInput {
    fn name(&self) -> String {
        String::from("string")
    }
    fn json_type(&self) -> String {
        String::from("string")
    }
    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
    fn clone_box(&self) -> Box<dyn InputInfo> {
        Box::new(self.clone())
    }
}
impl InputExtractor<String> for StringInput {
    fn parse_json(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(),
            _ => self.default.clone(),
        }
    }
    fn get_default(&self) -> String {
        self.default.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OptionsInput {
    pub options: Vec<String>,
    pub default: String,
}

impl InputInfo for OptionsInput {
    fn name(&self) -> String {
        String::from("options")
    }
    fn json_type(&self) -> String {
        String::from("string")
    }
    fn to_json(&self) -> Value {
        serde_json::to_value(self).unwrap()
    }
    fn clone_box(&self) -> Box<dyn InputInfo> {
        Box::new(self.clone())
    }
}
impl InputExtractor<String> for OptionsInput {
    fn parse_json(&self, value: &Value) -> String {
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
    fn get_default(&self) -> String {
        self.default.clone()
    }
}

pub struct GeneratorDimensionInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub data_info: Box<dyn InputInfo>,
}

impl Serialize for GeneratorDimensionInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let info = self.data_info.to_json();

        let mut state = serializer.serialize_struct("GeneratorDimensionInfo", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("data_info", &info)?;
        state.serialize_field("data_type", &(self.data_info.name()))?;
        state.serialize_field("json_type", &(self.data_info.json_type()))?;
        state.end()
    }
}

impl Clone for GeneratorDimensionInfo {
    fn clone(&self) -> Self {
        GeneratorDimensionInfo {
            name: self.name,
            description: self.description,
            data_info: self.data_info.clone_box(),
        }
    }
}

impl fmt::Debug for GeneratorDimensionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{name:{}, description: {}}}",
            self.name, self.description
        )
    }
}
