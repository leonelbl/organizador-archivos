use colored::*;
use std::io::{self, Write};

pub struct Output;

impl Output {
    pub fn new() -> Self {
        Self
    }

    pub fn files_found(&self, cantidad: usize, directorio: &str) {
        println!(
            "INFO: Se encontraron {} archivos en {}",
            cantidad.to_string().yellow(),
            directorio.cyan()
        );
    }

    pub fn confirm(&self) -> bool {
        print!("{} ", "¿Confirmas mover estos archivos?".bold().yellow());
        io::stdout().flush().unwrap();
        print!("[s/N]: ");
        io::stdout().flush().unwrap();

        let mut respuesta = String::new();
        if io::stdin().read_line(&mut respuesta).is_err() {
            return false;
        }
        respuesta.trim().to_lowercase() == "s"
    }

    pub fn finished(&self, movidos: usize, conflictos: usize, dry_run: bool) {
        if dry_run {
            println!(
                "\nSIMULADO: {} archivos hubieran sido movidos.",
                movidos.to_string().bright_blue().bold()
            );
        } else {
            println!(
                "\nFINALIZADO: Se movieron {} archivos{}.",
                movidos.to_string().yellow().bold(),
                if conflictos > 0 {
                    format!(" ({} conflictos resueltos)", conflictos.to_string().cyan())
                } else {
                    String::new()
                }
            );
        }
    }

    pub fn no_files_found(&self) {
        println!("{}", "INFO: No se encontraron archivos.".blue());
    }

    pub fn cancelled(&self) {
        println!("{}", "Operación cancelada.".yellow());
    }

    pub fn undo_start(&self, timestamp: &str, directorio: &str, cantidad: usize) {
        println!(
            "{} Revirtiendo operación del {}",
            "DESHACER:".bright_magenta().bold(),
            timestamp.cyan()
        );
        println!(
            "  {} archivos serán restaurados",
            cantidad.to_string().yellow()
        );
        println!("  Directorio: {}\n", directorio);
    }

    pub fn undo_finished(&self, revertidos: usize) {
        println!(
            "\nFINALIZADO: Se revirtieron {} archivos.",
            revertidos.to_string().yellow().bold()
        );
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}
