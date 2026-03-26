use std::fmt;

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
