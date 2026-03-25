use crate::shared::{JsonOutput, Logger, Notifier, OperationRecord, Output};
use crate::undo::History;
use super::{Mover, Scanner};
use super::cli::OrganizeArgs;

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

        let archivos = self.scanner.scan(&args.directorio, &extension, args.recursivo);

        if archivos.is_empty() {
            self.output.no_files_found();
            return Ok(());
        }

        if !args.yes && !args.dry_run {
            self.output.files_found(archivos.len(), &args.directorio.to_string_lossy());
            if !self.output.confirm() {
                self.output.cancelled();
                return Ok(());
            }
        }

        let (movidos, conflictos) = self.mover.mover(&archivos, &args.directorio, &extension, args.dry_run);

        if !args.dry_run && !movidos.is_empty() {
            let record = OperationRecord::new(
                &args.directorio.to_string_lossy(),
                &extension,
                movidos.clone(),
            );
            let history = History::new(args.undo_file.clone());
            let _ = history.save(&record);
        }

        self.logger.log(&args.directorio.to_string_lossy(), &extension, movidos.len(), conflictos);

        if args.json {
            let json = JsonOutput::new();
            if movidos.is_empty() && !args.dry_run {
                json.print_no_files();
            } else {
                json.print_result(movidos.len(), conflictos, args.dry_run);
            }
        } else {
            self.output.finished(movidos.len(), conflictos, args.dry_run);
        }

        if !args.dry_run && !movidos.is_empty() {
            self.notifier.notify(
                "Organizador de Archivos",
                &format!("Se movieron {} archivos a '{}'", movidos.len(), extension),
            );
        }

        Ok(())
    }
}
