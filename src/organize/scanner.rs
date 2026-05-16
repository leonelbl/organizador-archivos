use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Scanner;

impl Scanner {
    pub fn new() -> Self {
        Self
    }

    pub fn scan(&self, directorio: &Path, extension: &str, recursivo: bool) -> Vec<PathBuf> {
        let cleaned = extension.trim_start_matches('.');
        if cleaned.is_empty() || !cleaned.chars().all(|c| c.is_alphanumeric()) {
            return Vec::new();
        }
        let ext_filter = cleaned.to_lowercase();

        let walker = if recursivo {
            WalkDir::new(directorio)
        } else {
            WalkDir::new(directorio).max_depth(1)
        };

        walker
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().is_file()
                    && e.path()
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_lowercase() == ext_filter)
                        .unwrap_or(false)
            })
            .map(|e| e.path().to_path_buf())
            .collect()
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}
