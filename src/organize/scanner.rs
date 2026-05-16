use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const MAX_ARCHIVOS: usize = 100_000;

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

        let mut archivos: Vec<PathBuf> = walker
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let metadata = match e.metadata() {
                    Ok(m) => m,
                    Err(_) => return false,
                };
                !metadata.file_type().is_symlink()
                    && metadata.is_file()
                    && e.path()
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|s| s.to_lowercase() == ext_filter)
                        .unwrap_or(false)
            })
            .map(|e| e.path().to_path_buf())
            .take(MAX_ARCHIVOS + 1)
            .collect();

        if archivos.len() > MAX_ARCHIVOS {
            eprintln!(
                "ADVERTENCIA: Se limitó el escaneo a {} archivos. Use un directorio más específico.",
                MAX_ARCHIVOS
            );
            archivos.truncate(MAX_ARCHIVOS);
        }

        archivos
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}
