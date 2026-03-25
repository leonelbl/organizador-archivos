use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveRecord {
    pub origen: String,
    pub destino: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    pub timestamp: String,
    pub directorio: String,
    pub extension: String,
    pub movidos: Vec<MoveRecord>,
}

impl OperationRecord {
    pub fn new(directorio: &str, extension: &str, movidos: Vec<MoveRecord>) -> Self {
        Self {
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            directorio: directorio.to_string(),
            extension: extension.to_string(),
            movidos,
        }
    }
}
