use std::collections::HashMap;
use std::path::PathBuf;

use super::cli::OrganizeArgs;
use super::{Mover, Scanner};
use crate::shared::domain::MoveRecord;
use crate::shared::{BarraProgreso, JsonOutput, Logger, Notifier, OperationRecord, Output};
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
        let extensions = args.get_extensions()?;
        let directorio = args.validate_dir()?;

        let destino = args.destino.as_ref().map(|d| {
            let d = if d.is_relative() {
                directorio.join(d)
            } else {
                d.clone()
            };
            if !d.exists() {
                std::fs::create_dir_all(&d)
                    .map_err(|e| format!("No se pudo crear directorio de destino: {}", e))?;
            } else if !d.is_dir() {
                return Err("El destino especificado no es un directorio".to_string());
            }
            d.canonicalize()
                .map_err(|e| format!("Error al resolver ruta de destino: {}", e))
        }).transpose()?;

        if args.dry_run {
            return self.ejecutar_simulacion(args, &extensions, &directorio, destino.as_deref());
        }

        let mut total_movidos = 0;
        let mut total_conflictos = 0;
        let mut todos_movidos: Vec<MoveRecord> = Vec::new();

        for extension in &extensions {
            let archivos = self
                .scanner
                .scan(&directorio, extension, args.recursivo);

            if archivos.is_empty() {
                continue;
            }

            if !args.yes && total_movidos == 0 {
                self.output
                    .files_found(archivos.len(), &directorio.to_string_lossy());
                if !self.output.confirm() {
                    self.output.cancelled();
                    return Ok(());
                }
            }

            let usar_barra = archivos.len() >= 5;
            let (movidos, conflictos) = if usar_barra {
                let barra = BarraProgreso::new(archivos.len());
                let resultado = self.mover.mover_con_progreso(
                    &archivos,
                    &directorio,
                    extension,
                    destino.as_deref(),
                    &barra,
                );
                barra.finalizar();
                resultado
            } else {
                self.mover.mover(
                    &archivos,
                    &directorio,
                    extension,
                    false,
                    destino.as_deref(),
                )
            };

            total_movidos += movidos.len();
            total_conflictos += conflictos;
            todos_movidos.extend(movidos);
        }

        if total_movidos == 0 {
            if args.json {
                let json = JsonOutput::new();
                json.print_no_files();
            } else {
                self.output.no_files_found();
            }
            return Ok(());
        }

        let directorio_str = directorio
            .to_str()
            .ok_or_else(|| "La ruta del directorio contiene caracteres no UTF-8".to_string())?;

        let record = OperationRecord::new(
            directorio_str,
            &extensions.join(","),
            todos_movidos,
        );
        let history = History::new(args.undo_file.clone());
        let _ = history.save(&record);

        self.logger.log(
            directorio_str,
            &extensions.join(","),
            total_movidos,
            total_conflictos,
        );

        if args.json {
            let json = JsonOutput::new();
            json.print_result(total_movidos, total_conflictos, false);
        } else {
            self.output.finished(total_movidos, total_conflictos, false);
        }

        self.notifier.notify(
            "Organizador de Archivos",
            &format!("Se movieron {} archivos", total_movidos),
        );

        Ok(())
    }

    fn ejecutar_simulacion(
        &self,
        args: &OrganizeArgs,
        extensions: &[String],
        directorio: &std::path::Path,
        destino: Option<&std::path::Path>,
    ) -> Result<(), String> {
        let mut movimientos_por_ext: HashMap<String, Vec<(PathBuf, PathBuf)>> = HashMap::new();
        let mut total_archivos = 0;

        for extension in extensions {
            let archivos = self
                .scanner
                .scan(directorio, extension, args.recursivo);

            if archivos.is_empty() {
                continue;
            }

            let mut movimientos: Vec<(PathBuf, PathBuf)> = Vec::new();

            for archivo in &archivos {
                let destino = self.mover.calcular_destino(
                    archivo,
                    directorio,
                    extension,
                    destino,
                );
                movimientos.push((archivo.clone(), destino));
            }

            total_archivos += archivos.len();
            movimientos_por_ext
                .entry(extension.clone())
                .or_default()
                .extend(movimientos);
        }

        if total_archivos == 0 {
            self.output.no_files_found();
            return Ok(());
        }

        let directorio_base = destino
            .map(|d| d.to_string_lossy().to_string())
            .unwrap_or_else(|| directorio.to_string_lossy().to_string());

        self.output
            .mostrar_arbol_simulacion(&movimientos_por_ext, &directorio_base);
        self.output.finished(total_archivos, 0, true);

        Ok(())
    }
}
