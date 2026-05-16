use crate::shared::domain::OperationRecord;
use std::fs;
use std::path::{Path, PathBuf};

pub struct History {
    file_path: PathBuf,
}

impl History {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn save(&self, record: &OperationRecord) -> Result<(), String> {
        let json = serde_json::to_string(record).map_err(|e| e.to_string())?;
        fs::write(&self.file_path, json).map_err(|e| e.to_string())
    }

    pub fn get_last(&self) -> Result<Option<OperationRecord>, String> {
        if !self.file_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&self.file_path).map_err(|e| e.to_string())?;
        if content.trim().is_empty() {
            return Ok(None);
        }

        serde_json::from_str(&content)
            .map(Some)
            .map_err(|e| e.to_string())
    }

    pub fn revert(&self) -> Result<usize, String> {
        let record = self.get_last()?.ok_or("No hay historial para revertir")?;
        let directorio_base = Path::new(&record.directorio)
            .canonicalize()
            .map_err(|_| "El directorio del historial ya no existe".to_string())?;
        let mut revertidos = 0;

        for mov in record.movidos.iter().rev() {
            let origen = Path::new(&mov.destino);
            let destino = Path::new(&mov.origen);

            let origen_ok = match origen.canonicalize() {
                Ok(p) => p.starts_with(&directorio_base),
                Err(_) => false,
            };
            let destino_ok = match destino.canonicalize() {
                Ok(p) => p.starts_with(&directorio_base),
                Err(_) => false,
            };

            if !origen_ok && !destino_ok {
                eprintln!("ERROR: Path fuera del directorio permitido, se omite: {:?} -> {:?}", origen, destino);
                continue;
            }

            if origen.exists() {
                if let Some(parent) = destino.parent() {
                    let _ = fs::create_dir_all(parent);
                }
                if fs::rename(origen, destino).is_ok() {
                    revertidos += 1;
                }
            }
        }

        self.remove_last()?;
        Ok(revertidos)
    }

    fn remove_last(&self) -> Result<(), String> {
        if self.file_path.exists() {
            fs::remove_file(&self.file_path).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
