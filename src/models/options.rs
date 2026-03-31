use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionStrategy {
    Stable,
    Latest,
}

impl fmt::Display for VersionStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VersionStrategy::Stable => write!(f, "Stable (versions pinned)"),
            VersionStrategy::Latest => write!(f, "Latest (any)"),
        }
    }
}

impl VersionStrategy {
    /// Returns the pinned stable version constraint for a known package,
    /// or `"any"` if the package is unknown (fallback to latest).
    pub fn version_for(&self, pkg: &str) -> String {
        match self {
            VersionStrategy::Latest => "any".to_string(),
            VersionStrategy::Stable => stable_version(pkg)
                .unwrap_or("any")
                .to_string(),
        }
    }
}

fn stable_version(pkg: &str) -> Option<&'static str> {
    match pkg {
        // State management
        "flutter_riverpod" => Some("^2.5.1"),
        "riverpod_annotation" => Some("^2.3.5"),
        "riverpod_generator" => Some("^2.4.0"),
        "hooks_riverpod" => Some("^2.5.2"),
        "flutter_hooks" => Some("^0.20.5"),
        "flutter_bloc" => Some("^8.1.6"),
        "bloc" => Some("^8.1.4"),
        "get" => Some("^4.6.6"),
        "provider" => Some("^6.1.2"),
        // Routing
        "go_router" => Some("^14.6.2"),
        "auto_route" => Some("^9.2.2"),
        // Networking
        "dio" => Some("^5.7.0"),
        // Local storage
        "shared_preferences" => Some("^2.3.3"),
        "hive" => Some("^2.2.3"),
        "hive_flutter" => Some("^1.1.0"),
        "drift" => Some("^2.20.3"),
        // Code generation
        "freezed" => Some("^2.5.7"),
        "freezed_annotation" => Some("^2.4.4"),
        "json_serializable" => Some("^6.8.0"),
        "json_annotation" => Some("^4.9.0"),
        "build_runner" => Some("^2.4.13"),
        "flutter_gen" => Some("^5.7.0"),
        // Firebase
        "firebase_core" => Some("^3.8.1"),
        "firebase_auth" => Some("^5.3.4"),
        "cloud_firestore" => Some("^5.5.1"),
        "firebase_storage" => Some("^12.3.7"),
        "firebase_messaging" => Some("^15.1.6"),
        "cloud_functions" => Some("^5.1.5"),
        // Observability
        "sentry_flutter" => Some("^8.12.0"),
        "mixpanel_flutter" => Some("^2.3.3"),
        // Utilities
        "equatable" => Some("^2.0.5"),
        "dartz" => Some("^0.10.1"),
        "intl" => Some("any"),  // version pinned by flutter_localizations SDK
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateManagement {
    Riverpod,
    Bloc,
    GetX,
    Provider,
}

impl fmt::Display for StateManagement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateManagement::Riverpod => write!(f, "Riverpod"),
            StateManagement::Bloc => write!(f, "BLoC"),
            StateManagement::GetX => write!(f, "GetX"),
            StateManagement::Provider => write!(f, "Provider"),
        }
    }
}

impl StateManagement {
    pub fn packages(&self) -> Vec<&'static str> {
        match self {
            StateManagement::Riverpod => vec![
                "flutter_riverpod",
                "riverpod_annotation",
                "riverpod_generator",
            ],
            StateManagement::Bloc => vec!["flutter_bloc", "bloc"],
            StateManagement::GetX => vec!["get"],
            StateManagement::Provider => vec!["provider"],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingSolution {
    GoRouter,
    AutoRoute,
    GetXRouting,
    Navigator2,
}

impl fmt::Display for RoutingSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RoutingSolution::GoRouter => write!(f, "GoRouter"),
            RoutingSolution::AutoRoute => write!(f, "AutoRoute"),
            RoutingSolution::GetXRouting => write!(f, "GetX Routing"),
            RoutingSolution::Navigator2 => write!(f, "Navigator 2.0 (built-in)"),
        }
    }
}

impl RoutingSolution {
    pub fn package(&self) -> Option<&'static str> {
        match self {
            RoutingSolution::GoRouter => Some("go_router"),
            RoutingSolution::AutoRoute => Some("auto_route"),
            RoutingSolution::GetXRouting => Some("get"),
            RoutingSolution::Navigator2 => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtraPackage {
    Drift,
    Dio,
    Freezed,
    FlutterGen,
    Hive,
    SharedPreferences,
    Firebase,
    Equatable,
    Dartz,
    Intl,
}

impl fmt::Display for ExtraPackage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtraPackage::Drift => write!(f, "Drift (SQLite ORM)"),
            ExtraPackage::Dio => write!(f, "Dio (HTTP Client)"),
            ExtraPackage::Freezed => write!(f, "Freezed (Code Generation)"),
            ExtraPackage::FlutterGen => write!(f, "Flutter Gen (Asset Generation)"),
            ExtraPackage::Hive => write!(f, "Hive (Local Storage)"),
            ExtraPackage::SharedPreferences => write!(f, "Shared Preferences"),
            ExtraPackage::Firebase => write!(f, "Firebase Core"),
            ExtraPackage::Equatable => write!(f, "Equatable"),
            ExtraPackage::Dartz => write!(f, "Dartz (Functional Programming)"),
            ExtraPackage::Intl => write!(f, "Intl (Internationalization)"),
        }
    }
}

impl ExtraPackage {
    pub fn package(&self) -> &'static str {
        match self {
            ExtraPackage::Drift => "drift",
            ExtraPackage::Dio => "dio",
            ExtraPackage::Freezed => "freezed",
            ExtraPackage::FlutterGen => "flutter_gen",
            ExtraPackage::Hive => "hive",
            ExtraPackage::SharedPreferences => "shared_preferences",
            ExtraPackage::Firebase => "firebase_core",
            ExtraPackage::Equatable => "equatable",
            ExtraPackage::Dartz => "dartz",
            ExtraPackage::Intl => "intl",
        }
    }
}
