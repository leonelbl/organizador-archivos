//! Herramienta de CLI para organizar automáticamente archivos por extensión.
//! 
//! Esta herramienta escanea un directorio en busca de archivos con una extensión específica
//! y los mueve a un subdirectorio con el nombre de dicha extensión. Proporciona salida
//! colorida y notificaciones del sistema para una mejor experiencia de usuario.

use notify_rust::Notification;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;
use colored::*;

/// Envía notificaciones del sistema compatibles con múltiples entornos de escritorio.
/// 
/// Esta función intenta diferentes métodos de notificación en orden de preferencia:
/// 1. notify-rust (multiplataforma)
/// 2. notify-send (estándar de Linux)
/// 3. kdialog (KDE Plasma)
/// 4. zenity (GNOME/MATE/Cinnamon)
/// 5. Consola como alternativa
/// 
/// # Argumentos
/// 
/// * `title` - Título de la notificación
/// * `body` - Cuerpo/mensaje de la notificación
/// * `icon` - Nombre del icono a mostrar
fn send_notification(title: &str, body: &str, icon: &str) {
    // Intentar diferentes métodos de notificación según el entorno disponible
    
    // 1. Primero intentar con notify-rust (compatible con la mayoría de escritorios)
    match Notification::new()
        .summary(title)
        .body(body)
        .icon(icon)
        .timeout(5000)
        .show()
    {
        Ok(_) => return,
        Err(_) => {
            // Si notify-rust falla, intentar métodos alternativos
        }
    }
    
    // 2. Intentar con notify-send (disponible en la mayoría de sistemas Linux)
    if Command::new("which")
        .arg("notify-send")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        if let Ok(_) = Command::new("notify-send")
            .arg(title)
            .arg(body)
            .arg("--icon")
            .arg(icon)
            .arg("--expire-time")
            .arg("5000")
            .output()
        {
            return;
        }
    }
    
    // 3. Intentar con kdialog (para KDE)
    if Command::new("which")
        .arg("kdialog")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        if let Ok(_) = Command::new("kdialog")
            .arg("--title")
            .arg(title)
            .arg("--passivepopup")
            .arg(body)
            .arg("5")
            .output()
        {
            return;
        }
    }
    
    // 4. Intentar con zenity (para GNOME/MATE/Cinnamon)
    if Command::new("which")
        .arg("zenity")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        if let Ok(_) = Command::new("zenity")
            .arg("--info")
            .arg("--title")
            .arg(title)
            .arg("--text")
            .arg(body)
            .arg("--timeout")
            .arg("5")
            .output()
        {
            return;
        }
    }
    
    // 5. Si todo falla, imprimir en consola como alternativa
    println!("\n{} {}: {}", "NOTIFICATION:".bright_yellow().bold(), title, body);
}

/// Función principal que orquesta el proceso de organización de archivos.
/// 
/// # Argumentos
/// 
/// * `args[1]` - Directorio a escanear
/// * `args[2]` - Extensión de archivo a organizar (con o sin punto inicial)
/// 
/// # Ejemplo
/// 
/// ```bash
/// organizador-archivos ~/Downloads .MOV
/// ```
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        // Uso de .bright_red() y .bold()
        eprintln!("{} {} <directorio> <extension>", "Error:".bright_red().bold(), "Uso:".yellow());
        eprintln!("Ejemplo: {} /home/leonel/Downloads .MOV", args[0].cyan());
        return;
    }

    let ruta_entrada = &args[1];
    let ext_objetivo = args[2].trim_start_matches('.').to_lowercase();
    let path_origen = Path::new(ruta_entrada);

    if !path_origen.exists() || !path_origen.is_dir() {
        eprintln!("{}", "Error: El directorio origen no es válido o no existe.".red().bold());
        return;
    }

    let mut archivos_encontrados = Vec::new();

    for entrada in WalkDir::new(path_origen).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        let path = entrada.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if ext.to_lowercase() == ext_objetivo {
                    archivos_encontrados.push(path.to_path_buf());
                }
            }
        }
    }

    if archivos_encontrados.is_empty() {
        println!("{} No se encontraron archivos con la extensión: .{}", "info:".blue(), ext_objetivo.yellow());
        return;
    }

    // Mensaje con colores
    println!(
        "\n{} Se encontraron {} archivos {} en {}",
        "FOUND:".bright_green().bold(),
        archivos_encontrados.len().to_string().yellow(),
        format!(".{}", ext_objetivo).cyan(),
        ruta_entrada.blue()
    );

    print!("{} [s/N]: ", "¿Confirmas mover estos archivos?".bold());
    io::stdout().flush().unwrap();

    let mut respuesta = String::new();
    io::stdin().read_line(&mut respuesta).unwrap();

    if respuesta.trim().to_lowercase() != "s" {
        println!("{}", "Operación cancelada por el usuario.".yellow());
        return;
    }

    let mut ruta_destino = PathBuf::from(path_origen);
    ruta_destino.push(&ext_objetivo);

    if !ruta_destino.exists() {
        fs::create_dir(&ruta_destino).expect("No se pudo crear la carpeta");
        println!("{} Carpeta creada: {:?}", "OK:".green(), ruta_destino);
    }

    let mut movidos = 0;
    for ruta_archivo in archivos_encontrados {
        let nombre_archivo = ruta_archivo.file_name().unwrap();
        let mut destino_final = ruta_destino.clone();
        destino_final.push(nombre_archivo);

        match fs::rename(&ruta_archivo, destino_final) {
            Ok(_) => {
                // Imprimimos el nombre del archivo en verde claro
                println!("  {} {:?}", "✔".green(), nombre_archivo);
                movidos += 1;
            }
            Err(e) => eprintln!("  {} {:?}: {}", "✘".red(), nombre_archivo, e),
        }
    }

    println!(
        "\n{} Se movieron {} archivos a la carpeta {}.",
        "FINALIZADO:".on_green().white().bold(),
        movidos.to_string().yellow().bold(),
        ext_objetivo.cyan()
    );

    if movidos > 0 {
        // Enviar notificación nativa compatible con múltiples escritorios
        send_notification(
            "Organizador de Archivos",
            &format!("¡Éxito! Se movieron {} archivos a la carpeta '{}'.", movidos, ext_objetivo),
            "folder-download"
        );
    } else {
        send_notification(
            "Organizador de Archivos",
            "No se realizó ningún movimiento.",
            "info"
        );
    }
}