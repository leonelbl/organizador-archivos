use clap::{Parser, ValueHint};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "organizador-archivos",
    about = "Herramienta CLI para organizar archivos por extensión",
    version,
    author = "leonelbl"
)]
pub struct Cli {
    #[arg(help = "Directorio a escanear", value_hint = ValueHint::DirPath)]
    pub directorio: Option<PathBuf>,

    #[arg(
        short = 'e',
        long = "extension",
        help = "Extensión a organizar",
        value_name = "EXT"
    )]
    pub extension: Option<String>,

    #[arg(short = 'r', long = "recursivo", help = "Buscar en subdirectorios")]
    pub recursivo: bool,

    #[arg(short = 's', long = "si", help = "Confirmar automáticamente")]
    pub yes: bool,

    #[arg(short = 'n', long = "simular", help = "Simular sin mover archivos")]
    pub dry_run: bool,

    #[arg(short = 'j', long = "json", help = "Salida en formato JSON")]
    pub json: bool,

    #[arg(short = 'l', long = "log", help = "Archivo de log", value_hint = ValueHint::FilePath)]
    pub log_file: Option<PathBuf>,

    #[arg(short = 'd', long = "deshacer", help = "Deshacer última operación")]
    pub deshacer: bool,

    #[arg(long = "destino", help = "Directorio de destino personalizado", value_hint = ValueHint::DirPath)]
    pub destino: Option<PathBuf>,
}
