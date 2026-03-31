use crate::embedded::josoa;
use std::collections::HashMap;

pub struct JosoaTemplate;

impl JosoaTemplate {
    /// Retourne la map filename -> contenu Dart embarqué pour le template Josoa.
    pub fn templates(project_name: &str) -> HashMap<String, &'static str> {
        let mut map = HashMap::new();
        map.insert("main.dart".to_string(), josoa::MAIN_DART);
        map.insert("main_dev.dart".to_string(), josoa::MAIN_DEV_DART);
        map.insert("main_stg.dart".to_string(), josoa::MAIN_STG_DART);
        map.insert("main_prod.dart".to_string(), josoa::MAIN_PROD_DART);
        map.insert(
            format!("{}_app.dart", project_name),
            josoa::APP_DART,
        );
        map.insert("app_flavor.dart".to_string(), josoa::APP_FLAVOR);
        map.insert("app_env.dart".to_string(), josoa::APP_ENV);
        map.insert("app_env_fields.dart".to_string(), josoa::APP_ENV_FIELDS);
        map.insert("app_router.dart".to_string(), josoa::APP_ROUTER);
        map.insert("app_route.dart".to_string(), josoa::APP_ROUTE);
        map.insert("route_names.dart".to_string(), josoa::ROUTE_NAMES);
        map.insert("routes.dart".to_string(), josoa::ROUTES);
        map.insert(
            "go_router_delegate_listener.dart".to_string(),
            josoa::GO_ROUTER_DELEGATE_LISTENER,
        );
        map.insert(
            "app_startup_controller.dart".to_string(),
            josoa::APP_STARTUP_CONTROLLER,
        );
        map.insert(
            "app_startup_widget_wrapper.dart".to_string(),
            josoa::APP_STARTUP_WIDGET_WRAPPER,
        );
        map.insert(
            "app_exception.dart".to_string(),
            josoa::APP_EXCEPTION,
        );
        map.insert(
            "app_exception_code.dart".to_string(),
            josoa::APP_EXCEPTION_CODE,
        );
        map.insert(
            "error_placeholder.dart".to_string(),
            josoa::ERROR_PLACEHOLDER,
        );
        map.insert(
            "empty_placeholder.dart".to_string(),
            josoa::EMPTY_PLACEHOLDER,
        );
        map.insert(
            "loading_placeholder.dart".to_string(),
            josoa::LOADING_PLACEHOLDER,
        );
        map.insert(
            "error_logger.dart".to_string(),
            josoa::ERROR_LOGGER,
        );
        map.insert(
            "ref_extension.dart".to_string(),
            josoa::REF_EXTENSION,
        );
        map.insert(
            "string_extension.dart".to_string(),
            josoa::STRING_EXTENSION,
        );
        map.insert("app_sizes.dart".to_string(), josoa::APP_SIZES);
        map.insert(
            "string_hardcoded.dart".to_string(),
            josoa::STRING_HARDCODED,
        );
        map.insert("localization.dart".to_string(), josoa::LOCALIZATION);
        map.insert(
            "home_screen.dart".to_string(),
            josoa::HOME_SCREEN,
        );
        map.insert(
            "feature_screen.dart".to_string(),
            josoa::FEATURE_SCREEN,
        );
        map.insert(
            "feature_repository.dart".to_string(),
            josoa::FEATURE_REPOSITORY,
        );
        map.insert(
            "feature_service.dart".to_string(),
            josoa::FEATURE_SERVICE,
        );
        map.insert(
            "feature_model.dart".to_string(),
            josoa::FEATURE_MODEL,
        );
        map.insert(
            "feature_controller.dart".to_string(),
            josoa::FEATURE_CONTROLLER,
        );
        map
    }
}
