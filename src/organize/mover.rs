use std::fs;
use std::path::Path;
use chrono::Local;
use crate::shared::domain::MoveRecord;

pub struct Mover;

impl Mover {
    pub fn new() -> Self {
        Self
    }

    pub fn mover(
        &self,
        archivos: &[impl AsRef<Path>],
        directorio: &Path,
        extension: &str,
        dry_run: bool,
    ) -> (Vec<MoveRecord>, usize) {
        let ext_folder = extension.trim_start_matches('.').to_lowercase();
        let destino_dir = directorio.join(&ext_folder);

        if !dry_run && !destino_dir.exists() {
            if let Err(e) = fs::create_dir(&destino_dir) {
                eprintln!("ERROR: No se pudo crear carpeta {:?}: {}", destino_dir, e);
                return (Vec::new(), 0);
            }
        }

        let mut movidos = Vec::new();
        let mut conflictos = 0;

        for archivo in archivos {
            let archivo = archivo.as_ref();
            let nombre = archivo.file_name().unwrap_or_default();
            let mut destino = destino_dir.join(nombre);

            if destino.exists() {
                destino = self.generar_nombre_unico(&destino_dir, archivo);
                conflictos += 1;
            }

            if dry_run {
                movidos.push(MoveRecord {
                    origen: archivo.to_string_lossy().to_string(),
                    destino: destino.to_string_lossy().to_string(),
                });
            } else {
                match fs::rename(archivo, &destino) {
                    Ok(_) => {
                        movidos.push(MoveRecord {
                            origen: archivo.to_string_lossy().to_string(),
                            destino: destino.to_string_lossy().to_string(),
                        });
                    }
                    Err(e) => {
                        eprintln!("ERROR: {:?}: {}", nombre, e);
                    }
                }
            }
        }

        (movidos, conflictos)
    }

    fn generar_nombre_unico(&self, destino_dir: &Path, archivo: &Path) -> std::path::PathBuf {
        let ext = archivo
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{}", e))
            .unwrap_or_default();

        let stem = archivo
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("archivo");

        let mut contador = 1;
        loop {
            let nombre = format!("{}_{}{}", stem, contador, ext);
            let ruta = destino_dir.join(&nombre);
            if !ruta.exists() {
                return ruta;
            }
            contador += 1;
            if contador > 10000 {
                let timestamp = Local::now().format("%Y%m%d_%H%M%S");
                return destino_dir.join(format!("{}_{}{}", stem, timestamp, ext));
            }
        }
    }
}

impl Default for Mover {
    fn default() -> Self {
        Self::new()
    }
}
