use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct OrganizeArgs {
    pub directorio: PathBuf,
    pub extension: Option<String>,
    pub recursivo: bool,
    pub yes: bool,
    pub dry_run: bool,
    pub json: bool,
    pub log_file: Option<PathBuf>,
    pub undo_file: PathBuf,
}

impl OrganizeArgs {
    pub fn get_extension(&self) -> Result<String, String> {
        self.extension
            .clone()
            .filter(|e| !e.is_empty())
            .ok_or_else(|| "Debe especificar una extensión".to_string())
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
