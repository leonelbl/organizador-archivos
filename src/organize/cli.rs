use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct OrganizeArgs {
    pub directorio: PathBuf,
    pub extension: Vec<String>,
    pub recursivo: bool,
    pub yes: bool,
    pub dry_run: bool,
    pub json: bool,
    pub log_file: Option<PathBuf>,
    pub undo_file: PathBuf,
    pub destino: Option<PathBuf>,
}

impl OrganizeArgs {
    pub fn get_extensions(&self) -> Result<Vec<String>, String> {
        if self.extension.is_empty() {
            return Err("Debe especificar al menos una extensión".to_string());
        }

        let extensions: Vec<String> = self
            .extension
            .iter()
            .flat_map(|e| e.split(',').map(|s| s.trim().to_string()))
            .filter(|e| !e.is_empty())
            .collect();

        if extensions.is_empty() {
            return Err("Debe especificar al menos una extensión".to_string());
        }

        Ok(extensions)
    }

    pub fn validate_dir(&self) -> Result<(), String> {
        if !self.directorio.exists() || !self.directorio.is_dir() {
            return Err("El directorio no existe o no es válido".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UndoArgs {
    pub undo_file: PathBuf,
}
