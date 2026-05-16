use crate::shared::domain::OperationRecord;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub struct History {
    file_path: PathBuf,
}

impl History {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn save(&self, record: &OperationRecord) -> Result<(), String> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .map_err(|e| e.to_string())?;

        let json = serde_json::to_string(record).map_err(|e| e.to_string())?;
        writeln!(file, "{}", json).map_err(|e| e.to_string())
    }

    pub fn get_last(&self) -> Result<Option<OperationRecord>, String> {
        if !self.file_path.exists() {
            return Ok(None);
        }

        let file = File::open(&self.file_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        let mut last = None;
        for line in reader.lines().map_while(Result::ok) {
            if let Ok(record) = serde_json::from_str::<OperationRecord>(&line) {
                last = Some(record);
            }
        }

        Ok(last)
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
        if !self.file_path.exists() {
            return Ok(());
        }

        let file = File::open(&self.file_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);
        let mut lineas: Vec<String> = reader.lines().map_while(Result::ok).collect();

        if lineas.is_empty() {
            return Ok(());
        }

        lineas.pop();
        if lineas.is_empty() {
            fs::remove_file(&self.file_path).map_err(|e| e.to_string())?;
        } else {
            let mut file = File::create(&self.file_path).map_err(|e| e.to_string())?;
            writeln!(file, "{}", lineas.join("\n")).map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}
