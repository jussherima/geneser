use crate::embedded::code_with_andrea;
use std::collections::HashMap;

pub struct CodeWithAndreaTemplate;

impl CodeWithAndreaTemplate {
    /// Retourne la map filename -> contenu Dart embarqué pour le template CWA Medium.
    /// Utilisé uniquement pour la génération de fichiers (les .dart sont `include_str!` dans embedded/).
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
        map.insert(
            "app_exception.dart".to_string(),
            code_with_andrea::APP_EXCEPTION,
        );
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
        map.insert(
            "error_logger.dart".to_string(),
            code_with_andrea::ERROR_LOGGER,
        );
        map.insert(
            "ref_extension.dart".to_string(),
            code_with_andrea::REF_EXTENSION,
        );
        map.insert(
            "string_extension.dart".to_string(),
            code_with_andrea::STRING_EXTENSION,
        );
        map.insert("app_sizes.dart".to_string(), code_with_andrea::APP_SIZES);
        map.insert(
            "string_hardcoded.dart".to_string(),
            code_with_andrea::STRING_HARDCODED,
        );
        map.insert(
            "home_screen.dart".to_string(),
            code_with_andrea::HOME_SCREEN,
        );
        map.insert(
            "feature_screen.dart".to_string(),
            code_with_andrea::FEATURE_SCREEN,
        );
        map
    }
}
