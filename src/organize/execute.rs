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
        args.validate_dir()?;

        let mut total_movidos = 0;
        let mut total_conflictos = 0;
        let mut todos_movidos: Vec<MoveRecord> = Vec::new();

        for extension in &extensions {
            let archivos = self
                .scanner
                .scan(&args.directorio, extension, args.recursivo);

            if archivos.is_empty() {
                continue;
            }

            if !args.yes && !args.dry_run && total_movidos == 0 {
                self.output
                    .files_found(archivos.len(), &args.directorio.to_string_lossy());
                if !self.output.confirm() {
                    self.output.cancelled();
                    return Ok(());
                }
            }

            let usar_barra = archivos.len() >= 5;
            let (movidos, conflictos) = if usar_barra && !args.dry_run {
                let barra = BarraProgreso::new(archivos.len());
                let resultado =
                    self.mover
                        .mover_con_progreso(&archivos, &args.directorio, extension, &barra);
                barra.finalizar();
                resultado
            } else {
                self.mover
                    .mover(&archivos, &args.directorio, extension, args.dry_run)
            };

            total_movidos += movidos.len();
            total_conflictos += conflictos;
            todos_movidos.extend(movidos);
        }

        if total_movidos == 0 {
            self.output.no_files_found();
            return Ok(());
        }

        if !args.dry_run && !todos_movidos.is_empty() {
            let record = OperationRecord::new(
                &args.directorio.to_string_lossy(),
                &extensions.join(","),
                todos_movidos,
            );
            let history = History::new(args.undo_file.clone());
            let _ = history.save(&record);
        }

        self.logger.log(
            &args.directorio.to_string_lossy(),
            &extensions.join(","),
            total_movidos,
            total_conflictos,
        );

        if args.json {
            let json = JsonOutput::new();
            json.print_result(total_movidos, total_conflictos, args.dry_run);
        } else {
            self.output
                .finished(total_movidos, total_conflictos, args.dry_run);
        }

        if !args.dry_run && total_movidos > 0 {
            self.notifier.notify(
                "Organizador de Archivos",
                &format!("Se movieron {} archivos", total_movidos),
            );
        }

        Ok(())
    }
}
