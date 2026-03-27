use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirebaseLevel {
    None,
    AuthFirestore,
    Full,
}

impl fmt::Display for FirebaseLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FirebaseLevel::None => write!(f, "Aucun"),
            FirebaseLevel::AuthFirestore => write!(f, "Auth + Firestore"),
            FirebaseLevel::Full => write!(f, "Full (Auth + Firestore + Functions + Storage + Messaging)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservabilityLevel {
    None,
    Sentry,
    SentryAnalytics,
}

impl fmt::Display for ObservabilityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObservabilityLevel::None => write!(f, "Aucun"),
            ObservabilityLevel::Sentry => write!(f, "Sentry (Error tracking)"),
            ObservabilityLevel::SentryAnalytics => write!(f, "Sentry + Analytics (Mixpanel)"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodeWithAndreaOptions {
    pub firebase: FirebaseLevel,
    pub observability: ObservabilityLevel,
    pub features: Vec<String>,
}
