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
    pub dev_packages: Vec<String>,
    pub structure: HashMap<String, Vec<String>>,
    pub flags: HashMap<String, bool>,
    pub root_files: Vec<(String, String)>,
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
            dev_packages: Vec::new(),
            structure,
            flags: HashMap::new(),
            root_files: Vec::new(),
        }
    }

    pub fn with_flags(mut self, flags: HashMap<String, bool>) -> Self {
        self.flags = flags;
        self
    }

    pub fn with_root_files(mut self, root_files: Vec<(String, String)>) -> Self {
        self.root_files = root_files;
        self
    }

    pub fn with_dev_packages(mut self, dev_packages: Vec<String>) -> Self {
        self.dev_packages = dev_packages;
        self
    }
}
