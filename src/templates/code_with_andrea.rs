use crate::embedded::code_with_andrea;
use crate::models::code_with_andrea_options::{FirebaseLevel, CodeWithAndreaOptions, ObservabilityLevel};
use crate::models::template_config::TemplateConfig;
use std::collections::HashMap;

pub struct CodeWithAndreaTemplate;

impl CodeWithAndreaTemplate {
    pub fn build_config(project_name: &str, options: &CodeWithAndreaOptions) -> TemplateConfig {
        let mut flags = HashMap::new();
        flags.insert(
            "firebase".to_string(),
            options.firebase != FirebaseLevel::None,
        );
        flags.insert(
            "firebase_full".to_string(),
            options.firebase == FirebaseLevel::Full,
        );
        flags.insert(
            "sentry".to_string(),
            options.observability != ObservabilityLevel::None,
        );
        flags.insert(
            "analytics".to_string(),
            options.observability == ObservabilityLevel::SentryAnalytics,
        );

        let mut structure = HashMap::new();

        // Core structure
        structure.insert("lib".to_string(), vec!["main.dart".to_string()]);

        let app_file = format!("{}_app.dart", project_name);
        structure.insert("lib/src".to_string(), vec![app_file]);

        structure.insert(
            "lib/src/common_widgets".to_string(),
            vec![
                "error_placeholder.dart".to_string(),
                "empty_placeholder.dart".to_string(),
            ],
        );
        structure.insert(
            "lib/src/constants".to_string(),
            vec!["app_sizes.dart".to_string()],
        );
        structure.insert(
            "lib/src/exceptions".to_string(),
            vec![
                "app_exception.dart".to_string(),
                "app_exception_code.dart".to_string(),
            ],
        );
        structure.insert(
            "lib/src/localization".to_string(),
            vec!["string_hardcoded.dart".to_string()],
        );

        let mut routing_files = vec![
            "app_router.dart".to_string(),
            "app_route.dart".to_string(),
            "route_names.dart".to_string(),
            "routes.dart".to_string(),
        ];
        if options.observability != ObservabilityLevel::None {
            routing_files.push("go_router_delegate_listener.dart".to_string());
        }
        structure.insert("lib/src/routing".to_string(), routing_files);

        structure.insert(
            "lib/src/routing/initialization".to_string(),
            vec![
                "app_startup_controller.dart".to_string(),
                "app_startup_widget_wrapper.dart".to_string(),
            ],
        );

        structure.insert(
            "lib/src/services".to_string(),
            vec!["error_logger.dart".to_string()],
        );
        structure.insert(
            "lib/src/utils".to_string(),
            vec![
                "ref_extension.dart".to_string(),
                "string_extension.dart".to_string(),
            ],
        );
        structure.insert("lib/src/features".to_string(), vec![]);

        // Home feature (always present)
        structure.insert("lib/src/features/home".to_string(), vec![]);
        structure.insert(
            "lib/src/features/home/presentation".to_string(),
            vec!["home_screen.dart".to_string()],
        );

        // Extra features
        for feature in &options.features {
            let base = format!("lib/src/features/{}", feature);
            structure.insert(base.clone(), vec![]);
            structure.insert(
                format!("{}/presentation", base),
                vec![format!("{}_screen.dart", feature)],
            );
            structure.insert(format!("{}/application", base), vec![]);
            structure.insert(format!("{}/domain", base), vec![]);
            structure.insert(format!("{}/data", base), vec![]);
        }

        // Packages
        let mut packages = vec![
            "flutter_riverpod".to_string(),
            "riverpod_annotation".to_string(),
            "go_router".to_string(),
            "equatable".to_string(),
        ];

        let mut dev_packages = vec![
            "very_good_analysis".to_string(),
            "build_runner".to_string(),
            "riverpod_generator".to_string(),
            "riverpod_lint".to_string(),
            "custom_lint".to_string(),
        ];

        match options.firebase {
            FirebaseLevel::None => {}
            FirebaseLevel::AuthFirestore => {
                packages.extend([
                    "firebase_core".to_string(),
                    "firebase_auth".to_string(),
                    "cloud_firestore".to_string(),
                ]);
            }
            FirebaseLevel::Full => {
                packages.extend([
                    "firebase_core".to_string(),
                    "firebase_auth".to_string(),
                    "cloud_firestore".to_string(),
                    "cloud_functions".to_string(),
                    "firebase_storage".to_string(),
                    "firebase_messaging".to_string(),
                ]);
            }
        }

        match options.observability {
            ObservabilityLevel::None => {}
            ObservabilityLevel::Sentry => {
                packages.push("sentry_flutter".to_string());
            }
            ObservabilityLevel::SentryAnalytics => {
                packages.extend([
                    "sentry_flutter".to_string(),
                    "mixpanel_flutter".to_string(),
                ]);
            }
        }

        // Always add freezed for codegen
        packages.push("freezed_annotation".to_string());
        packages.push("json_annotation".to_string());
        dev_packages.extend([
            "freezed".to_string(),
            "json_serializable".to_string(),
        ]);

        // Root config files
        let root_files = vec![
            (".fvmrc".to_string(), code_with_andrea::FVMRC.to_string()),
            (
                "analysis_options.yaml".to_string(),
                code_with_andrea::ANALYSIS_OPTIONS.to_string(),
            ),
            (
                "lefthook.yaml".to_string(),
                code_with_andrea::LEFTHOOK.to_string(),
            ),
            (
                "commitlint.config.js".to_string(),
                code_with_andrea::COMMITLINT_CONFIG.to_string(),
            ),
            (
                "package.json".to_string(),
                code_with_andrea::PACKAGE_JSON.to_string(),
            ),
        ];

        let all_features: Vec<String> = std::iter::once("home".to_string())
            .chain(options.features.clone())
            .collect();

        TemplateConfig::new(
            "CodeWithAndrea",
            "Production-grade architecture inspired by CodeWithAndrea medium structure",
            all_features,
            "riverpod",
            "go_router",
            packages,
            structure,
        )
        .with_flags(flags)
        .with_root_files(root_files)
        .with_dev_packages(dev_packages)
    }

    /// Returns a map of filename -> template content for the CodeWithAndrea generator.
    pub fn templates(project_name: &str) -> HashMap<String, &'static str> {
        let mut map = HashMap::new();
        map.insert("main.dart".to_string(), code_with_andrea::MAIN_DART);
        map.insert(
            format!("{}_app.dart", project_name),
            code_with_andrea::APP_DART,
        );
        map.insert("app_router.dart".to_string(), code_with_andrea::APP_ROUTER);
        map.insert("app_route.dart".to_string(), code_with_andrea::APP_ROUTE);
        map.insert("route_names.dart".to_string(), code_with_andrea::ROUTE_NAMES);
        map.insert("routes.dart".to_string(), code_with_andrea::ROUTES);
        map.insert(
            "go_router_delegate_listener.dart".to_string(),
            code_with_andrea::GO_ROUTER_DELEGATE_LISTENER,
        );
        map.insert(
            "app_startup_controller.dart".to_string(),
            code_with_andrea::APP_STARTUP_CONTROLLER,
        );
        map.insert(
            "app_startup_widget_wrapper.dart".to_string(),
            code_with_andrea::APP_STARTUP_WIDGET_WRAPPER,
        );
        map.insert("app_exception.dart".to_string(), code_with_andrea::APP_EXCEPTION);
        map.insert(
            "app_exception_code.dart".to_string(),
            code_with_andrea::APP_EXCEPTION_CODE,
        );
        map.insert(
            "error_placeholder.dart".to_string(),
            code_with_andrea::ERROR_PLACEHOLDER,
        );
        map.insert(
            "empty_placeholder.dart".to_string(),
            code_with_andrea::EMPTY_PLACEHOLDER,
        );
        map.insert("error_logger.dart".to_string(), code_with_andrea::ERROR_LOGGER);
        map.insert("ref_extension.dart".to_string(), code_with_andrea::REF_EXTENSION);
        map.insert(
            "string_extension.dart".to_string(),
            code_with_andrea::STRING_EXTENSION,
        );
        map.insert("app_sizes.dart".to_string(), code_with_andrea::APP_SIZES);
        map.insert(
            "string_hardcoded.dart".to_string(),
            code_with_andrea::STRING_HARDCODED,
        );
        map.insert("home_screen.dart".to_string(), code_with_andrea::HOME_SCREEN);
        map.insert("feature_screen.dart".to_string(), code_with_andrea::FEATURE_SCREEN);
        map
    }
}
