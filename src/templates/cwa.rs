use crate::models::template_config::TemplateConfig;
use std::collections::HashMap;

pub struct CwaTemplate;

impl CwaTemplate {
    pub fn build_config(_project_name: &str, features: Vec<String>) -> TemplateConfig {
        let mut structure = HashMap::new();

        structure.insert("lib".to_string(), vec!["main.dart".to_string()]);
        structure.insert("lib/src".to_string(), vec![]);
        structure.insert(
            "lib/src/common_widgets".to_string(),
            vec![
                "async_value_widget.dart".to_string(),
                "empty_placeholder_widget.dart".to_string(),
            ],
        );
        structure.insert(
            "lib/src/constants".to_string(),
            vec!["app_sizes.dart".to_string()],
        );
        structure.insert(
            "lib/src/exceptions".to_string(),
            vec!["app_exception.dart".to_string()],
        );
        structure.insert(
            "lib/src/localization".to_string(),
            vec!["string_hardcoded.dart".to_string()],
        );
        structure.insert(
            "lib/src/routing".to_string(),
            vec![
                "app_router.dart".to_string(),
                "not_found_screen.dart".to_string(),
            ],
        );
        structure.insert(
            "lib/src/utils".to_string(),
            vec!["delay.dart".to_string(), "extensions.dart".to_string()],
        );
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

        let packages = vec![
            "flutter_riverpod".to_string(),
            "riverpod_annotation".to_string(),
            "riverpod_generator".to_string(),
            "go_router".to_string(),
            "freezed_annotation".to_string(),
            "json_annotation".to_string(),
        ];

        TemplateConfig::new(
            "CodeWithAndrea",
            "Feature-first architecture by Andrea Bizzotto",
            features,
            "riverpod",
            "go_router",
            packages,
            structure,
        )
    }
}
