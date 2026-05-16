use colored::*;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;

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
        let _ = io::stdout().flush();
        print!("[s/N]: ");
        let _ = io::stdout().flush();

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

    pub fn mostrar_arbol_simulacion(
        &self,
        archivos: &HashMap<String, Vec<(PathBuf, PathBuf)>>,
        directorio_base: &str,
    ) {
        println!("\n{}", "═".repeat(50).bright_blue());
        println!(
            "{} {} -{}",
            "SIMULACIÓN".bright_green().bold(),
            "Cambios que se realizarán".white(),
            "═".repeat(16).bright_blue()
        );
        println!("{}", "═".repeat(50).bright_blue());

        println!("\n{}", directorio_base.cyan().bold());
        println!("{}\n", "─".repeat(50).bright_black());

        for (extension, movimientos) in archivos {
            let carpeta = format!("📁 {}/", extension).yellow().bold();
            println!("  {}", carpeta);

            for (origen, destino) in movimientos {
                let nombre_origen = origen.file_name().and_then(|n| n.to_str()).unwrap_or("?");
                let nombre_destino = destino.file_name().and_then(|n| n.to_str()).unwrap_or("?");

                println!(
                    "  {} {} {} {}",
                    "├──".bright_black(),
                    nombre_origen.white(),
                    "→".bright_green(),
                    nombre_destino.bright_cyan()
                );
            }
            println!("{}\n", "  ".white());
        }

        let total: usize = archivos.values().map(|v| v.len()).sum();
        println!(
            "{} {} {}",
            "Total:".white(),
            total.to_string().yellow().bold(),
            "archivos".white()
        );
        println!("{}\n", "─".repeat(50).bright_black());
    }
}

impl Default for Output {
    fn default() -> Self {
        Self::new()
    }
}
