use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize};

pub mod dimensions;

use serde_json::Value;
use skia_safe::Canvas;

pub type Generator =
    fn(canvas: &mut Canvas, width: i32, height: i32, seed: u64, dimensions: HashMap<String, Value>);

#[derive(Serialize, Clone)]
pub struct GeneratorInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub author: &'static str,
    pub created_at: &'static str,
    pub dimensions: Vec<dimensions::GeneratorDimensionInfo>,

    #[serde(skip)]
    pub generate: Generator,
}

impl fmt::Debug for GeneratorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ name: {}, description: {} }}",
            self.name, self.description
        )
    }
}

#[derive(Serialize, Debug)]
pub struct GeneratorGroup {
    group_name: &'static str,
    generators: Vec<GeneratorInfo>,
    sub_groups: Vec<GeneratorGroup>,
}

impl GeneratorGroup {
    pub fn get_name(&self) -> &'static str {
        self.group_name
    }
    pub fn get_generators(&self) -> &Vec<GeneratorInfo> {
        &self.generators
    }
    pub fn get_sub_groups(&self) -> &Vec<GeneratorGroup> {
        &self.sub_groups
    }

    /// Flattens all generators and sub-generators in the group into a single vec
    pub fn flatten(&self) -> Vec<GeneratorInfo> {
        let mut list: Vec<GeneratorInfo> = vec![]; //self.generators.to_vec();

        for generator in self.generators.iter() {
            list.push(generator.clone());
        }

        for sub_group in self.sub_groups.iter() {
            for generator in sub_group.flatten() {
                list.push(generator);
            }
        }

        list
    }
}

pub struct GeneratorGroupBuilder {
    name: &'static str,
    generators: Vec<GeneratorInfo>,
    sub_groups: Vec<GeneratorGroupBuilder>,
}

impl GeneratorGroupBuilder {
    pub fn new(name: &'static str) -> GeneratorGroupBuilder {
        GeneratorGroupBuilder {
            name: name,
            generators: Vec::new(),
            sub_groups: Vec::new(),
        }
    }

    /// fluent api to add a sub-group
    pub fn add_sub_group(mut self, sub_group: GeneratorGroupBuilder) -> Self {
        self.sub_groups.push(sub_group);
        self
    }

    /// fluent api to add a generator
    pub fn add_generator(mut self, generator: GeneratorInfo) -> Self {
        self.generators.push(generator);
        self
    }

    pub fn finish(&self) -> GeneratorGroup {
        GeneratorGroup {
            group_name: self.name,
            generators: self.generators.to_vec(),
            sub_groups: self
                .sub_groups
                .iter()
                .map(|builder| builder.finish())
                .collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct GenerationRequest {
    pub generator_type: &'static str,
    pub seed: Option<u64>,
    pub width: u32,
    pub height: u32,
    pub dimension_values: HashMap<String, String>,
}
