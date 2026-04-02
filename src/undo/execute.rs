use super::History;
use crate::organize::cli::UndoArgs;
use crate::shared::{Notifier, Output};

pub struct UndoCmd {
    output: Output,
    notifier: Notifier,
}

impl UndoCmd {
    pub fn new() -> Self {
        Self {
            output: Output::new(),
            notifier: Notifier::new(),
        }
    }

    pub fn execute(&self, args: &UndoArgs) -> Result<(), String> {
        let history = History::new(args.undo_file.clone());
        let record = history
            .get_last()?
            .ok_or("No hay historial para revertir")?;

        self.output
            .undo_start(&record.timestamp, &record.directorio, record.movidos.len());

        let revertidos = history.revert()?;

        self.output.undo_finished(revertidos);

        self.notifier.notify(
            "Organizador de Archivos",
            &format!("Se revirtieron {} archivos", revertidos),
        );

        Ok(())
    }
}

impl Default for UndoCmd {
    fn default() -> Self {
        Self::new()
    }
}
