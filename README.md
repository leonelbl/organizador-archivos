# Organizador de Archivos

Una herramienta CLI en Rust para organizar automáticamente archivos por extensión.

## 🚀 Características

- **Organización automática**: Mueve archivos por extensión a subcarpetas
- **Interfaz amigable**: Salida colorida y confirmación interactiva
- **Notificaciones multi-escritorio**: Compatible con COSMIC, GNOME, KDE, y más
- **Seguro**: Pide confirmación antes de mover archivos
- **Rápido**: Escaneo eficiente de directorios

## 📦 Instalación

### Desde el código fuente

```bash
git clone https://github.com/tu-usuario/organizador-archivos.git
cd organizador-archivos
cargo build --release
```

El binario compilado estará en `target/release/organizador-archivos`.

## 🛠️ Uso

```bash
organizador-archivos <directorio> <extensión>
```

### Ejemplos

```bash
# Organizar archivos .MOV en Downloads
organizador-archivos ~/Downloads .MOV

# Organizar archivos .pdf en Documents
organizador-archivos ~/Documents .pdf

# Organizar archivos .zip en el directorio actual
organizador-archivos . .zip
```

## 📋 Cómo funciona

1. **Escaneo**: Busca archivos con la extensión especificada en el directorio
2. **Confirmación**: Muestra los archivos encontrados y pide confirmación
3. **Organización**: Crea una carpeta con el nombre de la extensión (si no existe)
4. **Movimiento**: Mueve todos los archivos encontrados a la carpeta correspondiente
5. **Notificación**: Envía una notificación del sistema con el resultado

## 🔧 Dependencias

- `colored`: Salida colorida en terminal
- `notify-rust`: Notificaciones del sistema
- `walkdir`: Escaneo eficiente de directorios
- `directories`: Utilidades de directorios del sistema

## 🖥️ Compatibilidad

### Sistemas de notificación soportados

- **notify-rust**: Compatible con la mayoría de escritorios Linux
- **notify-send**: Sistemas Linux estándar
- **kdialog**: KDE Plasma
- **zenity**: GNOME, MATE, Cinnamon
- **Fallback**: Salida por consola si no hay sistema de notificación

### Sistemas operativos

- ✅ Linux (todas las distribuciones)
- 🔄 macOS (parcialmente compatible)
- ❌ Windows (no compatible actualmente)

## 🎯 Ejemplo de uso

```bash
$ organizador-archivos ~/Downloads .MOV

FOUND: Se encontraron 5 archivos .MOV en /home/user/Downloads
¿Confirmas mover estos archivos? [s/N]: s
OK: Carpeta creada: /home/user/Downloads/MOV
  ✔ video1.MOV
  ✔ video2.MOV
  ✔ video3.MOV
  ✔ video4.MOV
  ✔ video5.MOV

FINALIZADO: Se movieron 5 archivos a la carpeta MOV.
```

## 🤝 Contribuir

Las contribuciones son bienvenidas. Por favor:

1. Fork del proyecto
2. Crea una rama (`git checkout -b feature/nueva-caracteristica`)
3. Commit tus cambios (`git commit -am 'Añadir nueva característica'`)
4. Push a la rama (`git push origin feature/nueva-caracteristica`)
5. Abre un Pull Request

## 📝 Licencia

Este proyecto está bajo la Licencia MIT.

## 🐛 Issues

Si encuentras algún bug o tienes sugerencias, por favor abre un issue en [GitHub Issues](https://github.com/tu-usuario/organizador-archivos/issues).

## 📈 Roadmap

- [ ] Soporte para Windows
- [ ] Modo recursivo (subdirectorios)
- [ ] Archivo de configuración
- [ ] Múltiples extensiones simultáneas
- [ ] Modo "dry run" (simulación sin mover)

---

**Creado con Rust**