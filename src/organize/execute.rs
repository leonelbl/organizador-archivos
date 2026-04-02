use std::collections::HashMap;
use std::path::PathBuf;

use super::cli::OrganizeArgs;
use super::{Mover, Scanner};
use crate::shared::{JsonOutput, Logger, Notifier, OperationRecord, Output};
use crate::undo::History;

pub struct OrganizeCommand {
    scanner: Scanner,
    mover: Mover,
    output: Output,
    notifier: Notifier,
    logger: Logger,
}

impl OrganizeCommand {
    pub fn new(log_file: Option<std::path::PathBuf>) -> Self {
        Self {
            scanner: Scanner::new(),
            mover: Mover::new(),
            output: Output::new(),
            notifier: Notifier::new(),
            logger: Logger::new(log_file),
        }
    }

    pub fn execute(&self, args: &OrganizeArgs) -> Result<(), String> {
        let extension = args.get_extension()?;
        args.validate_dir()?;

        let archivos = self
            .scanner
            .scan(&args.directorio, &extension, args.recursivo);

        if archivos.is_empty() {
            self.output.no_files_found();
            return Ok(());
        }

        if args.dry_run {
            let mut movimientos_por_ext: HashMap<String, Vec<(PathBuf, PathBuf)>> = HashMap::new();

            for archivo in &archivos {
                let ext_folder = extension.trim_start_matches('.').to_lowercase();
                let destino_dir = args.directorio.join(&ext_folder);
                let nombre = archivo.file_name().unwrap_or_default();
                let mut destino = destino_dir.join(nombre);

                if destino.exists() {
                    destino = self.mover.generar_nombre_unico(&destino_dir, archivo);
                }

                movimientos_por_ext
                    .entry(ext_folder)
                    .or_default()
                    .push((archivo.clone(), destino));
            }

            self.output
                .mostrar_arbol_simulacion(&movimientos_por_ext, &args.directorio.to_string_lossy());

            self.output.finished(archivos.len(), 0, true);
            return Ok(());
        }

        if !args.yes {
            self.output
                .files_found(archivos.len(), &args.directorio.to_string_lossy());
            if !self.output.confirm() {
                self.output.cancelled();
                return Ok(());
            }
        }

        let (movidos, conflictos) =
            self.mover
                .mover(&archivos, &args.directorio, &extension, false);

        if !movidos.is_empty() {
            let record = OperationRecord::new(
                &args.directorio.to_string_lossy(),
                &extension,
                movidos.clone(),
            );
            let history = History::new(args.undo_file.clone());
            let _ = history.save(&record);
        }

        self.logger.log(
            &args.directorio.to_string_lossy(),
            &extension,
            movidos.len(),
            conflictos,
        );

        if args.json {
            let json = JsonOutput::new();
            if movidos.is_empty() {
                json.print_no_files();
            } else {
                json.print_result(movidos.len(), conflictos, false);
            }
        } else {
            self.output.finished(movidos.len(), conflictos, false);
        }

        if !movidos.is_empty() {
            self.notifier.notify(
                "Organizador de Archivos",
                &format!("Se movieron {} archivos a '{}'", movidos.len(), extension),
            );
        }

        Ok(())
    }
}
