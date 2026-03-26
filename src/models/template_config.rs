use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub name: String,
    pub description: String,
    pub features: Vec<String>,
    pub state_management: String,
    pub routing: String,
    pub packages: Vec<String>,
    pub structure: HashMap<String, Vec<String>>,
}

impl TemplateConfig {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        features: Vec<String>,
        state_management: impl Into<String>,
        routing: impl Into<String>,
        packages: Vec<String>,
        structure: HashMap<String, Vec<String>>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            features,
            state_management: state_management.into(),
            routing: routing.into(),
            packages,
            structure,
        }
    }
}
