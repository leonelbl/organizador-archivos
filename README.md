# Organizador de Archivos

Herramienta CLI en Rust para organizar automáticamente archivos por extensión.

## Características

- **Feature-Based Architecture**: Código modular y extensible
- **Comando por defecto**: `organizar` se ejecuta sin necesidad de especificarlo
- **Modo recursivo**: Busca en subdirectorios
- **Simulación**: Previsualiza archivos sin mover
- **Resolución de conflictos**: Nombres duplicados se renombran automáticamente
- **Deshacer**: Revierte movimientos realizados
- **Salida JSON**: Para integración con scripts
- **Log persistente**: Registro de operaciones
- **Interfaz amigable**: Colores y confirmación interactiva
- **Notificaciones multiplataforma**: GNOME, KDE, y más

## Instalación

```bash
git clone https://github.com/leonelbl/organizador-archivos.git
cd organizador-archivos
cargo build --release
```

El binario estará en `target/release/organizador-archivos`.

## Arquitectura

```
src/
├── main.rs              # Entry point
├── cli.rs               # CLI con argumentos
├── shared/              # Código compartido
│   ├── domain.rs        # Tipos (MoveRecord, OperationRecord)
│   ├── json.rs          # Serialización JSON
│   ├── log.rs           # Logging
│   ├── notification.rs  # Notificaciones del sistema
│   └── output.rs        # Salida con colores
├── organize/            # Feature: organizar
│   ├── cli.rs           # Args del comando
│   ├── execute.rs       # Lógica principal
│   ├── mover.rs         # Movimiento de archivos
│   └── scanner.rs       # Escaneo de directorios
└── undo/                # Feature: deshacer
    ├── execute.rs       # Lógica de reversión
    └── history.rs       # Manejo de historial
```

## Uso

```bash
organizador-archivos <DIRECTORIO> [OPTIONS]
```

**Opciones:**
```
-e, --extension <EXT>   Extensión a organizar
-r, --recursivo         Buscar en subdirectorios
-s, --si                Confirmar automáticamente
-n, --simular           Simular sin mover archivos
-j, --json              Salida en formato JSON
-l, --log <ARCHIVO>     Guardar registro en archivo
-d, --deshacer          Deshacer última operación
    --destino <DIR>       Directorio de destino personalizado
-h, --help              Mostrar ayuda
```

## Ejemplos

```bash
# Organizar una extensión (comando 'organizar' es implícito)
organizador-archivos ~/Descargas -e pdf

# Con confirmación automática
organizador-archivos ~/Descargas -e mp3 -s

# Modo recursivo
organizador-archivos ~/Descargas -e mp4 -r -s

# Simular sin mover
organizador-archivos ~/Descargas -e mp4 -n

# Salida JSON
organizador-archivos ~/Descargas -e pdf -j -s

# Log de operaciones
organizador-archivos ~/Descargas -e zip -l ~/.organizador.log -s

# Deshacer última operación
organizador-archivos -d

# Destino personalizado
organizador-archivos ~/Descargas -e pdf -o ~/Documentos -s
```

## Cómo funciona

1. **Escaneo**: Busca archivos con la extensión especificada
2. **Confirmación**: Muestra archivos y pide confirmación (excepto con `-s`)
3. **Organización**: Crea carpeta con el nombre de la extensión
4. **Movimiento**: Mueve archivos, renombrando conflictos automáticamente
5. **Registro**: Guarda historial para posible reversión
6. **Notificación**: Envía notificación del sistema

## Conflictos de nombres

Si ya existe un archivo con el mismo nombre en el destino, se renombra automáticamente:

```
archivo.txt      → archivo_1.txt
archivo.txt      → archivo_2.txt
```

## Salida JSON

Útil para integración con scripts:

```json
{
  "success": true,
  "archivos_encontrados": 3,
  "archivos_movidos": 3,
  "conflictos_resueltos": 0,
  "archivos": [],
  "mensaje": "Se movieron 3 archivos"
}
```

## Historial

El historial se guarda en `.organizador_history.json`. Para revertir:

```bash
organizador-archivos -d
```

## Dependencias

- `clap`: Parsing de argumentos CLI
- `colored`: Salida con colores en terminal
- `chrono`: Timestamps para logs
- `notify-rust`: Notificaciones del sistema
- `serde`: Serialización JSON
- `walkdir`: Escaneo eficiente de directorios

## Compatibilidad

### Notificaciones

- **notify-rust**: Compatible con la mayoría de escritorios Linux
- **notify-send**: Sistemas Linux estándar
- **kdialog**: KDE Plasma
- **zenity**: GNOME, MATE, Cinnamon
- **Fallback**: Salida por consola si no hay sistema de notificación

### Sistemas operativos

- ✅ Linux (todas las distribuciones)
- 🔄 macOS (parcialmente compatible)
- ❌ Windows (no compatible actualmente)

## Licencia

MIT License - ver archivo [LICENSE](LICENSE)
