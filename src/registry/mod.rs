pub mod loader;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

const REGISTRY_URL: &str =
    "https://raw.githubusercontent.com/redsmite/geneser-registry/main/registry.json";
const CACHE_TTL_SECS: u64 = 86400; // 24h

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub name: String,
    pub description: String,
    pub repo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Registry {
    pub templates: Vec<RegistryEntry>,
}

pub fn templates_dir() -> PathBuf {
    let base = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("geneser")
        .join("templates");
    base
}

fn cache_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("geneser")
        .join("registry-cache.json")
}

fn is_cache_fresh(path: &PathBuf) -> bool {
    if let Ok(meta) = fs::metadata(path) {
        if let Ok(modified) = meta.modified() {
            if let Ok(elapsed) = SystemTime::now().duration_since(modified) {
                return elapsed < Duration::from_secs(CACHE_TTL_SECS);
            }
        }
    }
    false
}

/// Fetch the remote registry, with 24h cache. Falls back to cached version on network error.
pub fn fetch_registry() -> Result<Registry> {
    let cache = cache_path();

    // Return fresh cache if available
    if is_cache_fresh(&cache) {
        if let Ok(data) = fs::read_to_string(&cache) {
            if let Ok(reg) = serde_json::from_str::<Registry>(&data) {
                return Ok(reg);
            }
        }
    }

    // Try network
    match reqwest::blocking::get(REGISTRY_URL) {
        Ok(resp) => {
            let text = resp.text()?;
            let reg: Registry = serde_json::from_str(&text)?;
            // Persist cache
            if let Some(parent) = cache.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::write(&cache, &text);
            Ok(reg)
        }
        Err(_) => {
            // Fallback to stale cache
            if let Ok(data) = fs::read_to_string(&cache) {
                if let Ok(reg) = serde_json::from_str::<Registry>(&data) {
                    return Ok(reg);
                }
            }
            Ok(Registry { templates: vec![] })
        }
    }
}
