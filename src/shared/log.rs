use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;

pub struct Logger {
    file_path: Option<PathBuf>,
}

impl Logger {
    pub fn new(file_path: Option<PathBuf>) -> Self {
        Self { file_path }
    }

    pub fn log(&self, directorio: &str, extension: &str, movidos: usize, conflictos: usize) {
        if let Some(ref path) = self.file_path {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
            {
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                let _ = writeln!(
                    file,
                    "[{}] Directorio: {}, Extensión: {}, Movidos: {}, Conflictos: {}",
                    timestamp, directorio, extension, movidos, conflictos
                );
            }
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self { file_path: None }
    }
}
