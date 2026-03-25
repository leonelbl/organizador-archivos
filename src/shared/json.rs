use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JsonResult {
    pub success: bool,
    pub archivos_encontrados: usize,
    pub archivos_movidos: usize,
    pub conflictos_resueltos: usize,
    pub mensaje: String,
}

pub struct JsonOutput;

impl JsonOutput {
    pub fn new() -> Self {
        Self
    }

    pub fn print_result(&self, movidos: usize, conflictos: usize, dry_run: bool) {
        let mensaje = if dry_run {
            format!("Simulación: {} archivos hubieran sido movidos", movidos)
        } else {
            format!("Se movieron {} archivos", movidos)
        };

        let resultado = JsonResult {
            success: !dry_run || movidos > 0,
            archivos_encontrados: movidos,
            archivos_movidos: movidos,
            conflictos_resueltos: conflictos,
            mensaje,
        };

        println!(
            "{}",
            serde_json::to_string_pretty(&resultado).unwrap_or_default()
        );
    }

    pub fn print_no_files(&self) {
        let resultado = JsonResult {
            success: true,
            archivos_encontrados: 0,
            archivos_movidos: 0,
            conflictos_resueltos: 0,
            mensaje: "No se encontraron archivos".to_string(),
        };
        println!(
            "{}",
            serde_json::to_string_pretty(&resultado).unwrap_or_default()
        );
    }
}

impl Default for JsonOutput {
    fn default() -> Self {
        Self::new()
    }
}
