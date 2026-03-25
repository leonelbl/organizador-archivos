mod cli;
mod organize;
mod shared;
mod undo;

use clap::Parser;
use cli::Cli;
use organize::{OrganizeArgs, OrganizeCommand};
use undo::UndoArgs;
use undo::UndoCmd;

fn main() {
    let cli = Cli::parse();

    if cli.deshacer {
        let args = UndoArgs {
            undo_file: std::path::PathBuf::from(".organizador_history.json"),
        };
        let cmd = UndoCmd::new();
        if let Err(e) = cmd.execute(&args) {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    } else {
        let args = OrganizeArgs {
            directorio: cli.directorio,
            extension: cli.extension,
            recursivo: cli.recursivo,
            yes: cli.yes,
            dry_run: cli.dry_run,
            json: cli.json,
            log_file: cli.log_file,
            undo_file: std::path::PathBuf::from(".organizador_history.json"),
        };
        let cmd = OrganizeCommand::new(args.log_file.clone());
        if let Err(e) = cmd.execute(&args) {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}
