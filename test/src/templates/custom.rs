use crate::models::options::{ExtraPackage, RoutingSolution, StateManagement};
use crate::models::template_config::TemplateConfig;
use std::collections::HashMap;

pub struct CustomTemplate;

impl CustomTemplate {
    pub fn build_config(
        _project_name: &str,
        features: Vec<String>,
        state_management: StateManagement,
        routing: RoutingSolution,
        extras: Vec<ExtraPackage>,
    ) -> TemplateConfig {
        let mut packages = Vec::new();

        for pkg in state_management.packages() {
            packages.push(pkg.to_string());
        }

        if let Some(router_pkg) = routing.package() {
            packages.push(router_pkg.to_string());
        }

        for extra in extras {
            packages.push(extra.package().to_string());
        }

        let mut structure = HashMap::new();
        structure.insert("lib".to_string(), vec!["main.dart".to_string()]);
        structure.insert("lib/src/features".to_string(), vec![]);

        for feature in &features {
            let base = format!("lib/src/features/{}", feature);
            structure.insert(base.clone(), vec![]);
            structure.insert(
                format!("{}/presentation", base),
                vec![format!("{}_screen.dart", feature)],
            );
            structure.insert(
                format!("{}/application", base),
                vec![format!("{}_service.dart", feature)],
            );
            structure.insert(
                format!("{}/domain", base),
                vec![format!("{}_model.dart", feature)],
            );
            structure.insert(
                format!("{}/data", base),
                vec![format!("{}_repository.dart", feature)],
            );
        }

        TemplateConfig::new(
            "Custom",
            "Custom architecture",
            features,
            state_management.to_string(),
            routing.to_string(),
            packages,
            structure,
        )
    }
}
