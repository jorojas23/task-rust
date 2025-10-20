# Task-Rust: Gestor de Tareas en la Línea de Comandos

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

Una sencilla pero potente aplicación de línea de comandos (CLI) para gestionar tus tareas diarias. Escrita en Rust, esta herramienta te permite agregar, listar, actualizar, eliminar y cambiar el estado de tus tareas directamente desde la terminal.

## ✨ Características

- **Agregar tareas**: Añade nuevas tareas con un título y una descripción detallada.
- **Listar tareas**: Visualiza todas tus tareas en una tabla clara y organizada.
- **Actualizar tareas**: Modifica el título y/o la descripción de una tarea existente por su ID.
- **Eliminar tareas**: Borra tareas que ya no necesites usando su ID.
- **Cambiar estado**: Actualiza el progreso de una tarea (ej. "adelantando", "completada").
- **Persistencia local**: Tus tareas se guardan localmente en un archivo `tasks.json`, por lo que no perderás tu información.

## 📋 Requisitos

- Rust y Cargo (versión 1.78 o superior, por la `edition = "2024"`).

## 🚀 Instalación y Compilación

1.  Clona este repositorio en tu máquina local:
    ```bash
    git clone <URL_DE_TU_REPOSITORIO>
    cd task-rust
    ```

2.  Compila el proyecto en modo de lanzamiento (release) para obtener el mejor rendimiento:
    ```bash
    cargo build --release
    ```
    El ejecutable se encontrará en `./target/release/task-rust`.

3.  (Opcional) Para un acceso más fácil, puedes mover el ejecutable a un directorio en tu `PATH`:
    ```bash
    sudo mv ./target/release/task-rust /usr/local/bin/task-rust
    ```

## 💻 Uso

Puedes ejecutar los comandos usando `cargo run --` o directamente con el ejecutable compilado.

### Agregar una nueva tarea

```bash
# Sintaxis
cargo run -- add --title "<TÍTULO>" --description "<DESCRIPCIÓN>"

# Ejemplo
cargo run -- add --title "Estudiar Rust" --description "Leer el capítulo sobre ownership y borrowing."
```

### Listar todas las tareas

Muestra todas las tareas guardadas en una tabla.

```bash
cargo run -- list
```

### Actualizar una tarea existente

Puedes actualizar el título, la descripción o ambos.

```bash
# Sintaxis
cargo run -- update --id <ID> --title "[NUEVO_TÍTULO]" --description "[NUEVA_DESCRIPCIÓN]"

# Ejemplo (actualizando solo el título)
cargo run -- update --id 1 --title "Estudiar Rust a fondo"
```

### Eliminar una tarea

```bash
# Sintaxis
cargo run -- delete --id <ID>

# Ejemplo
cargo run -- delete --id 3
```

### Cambiar el estado de una tarea

Los estados sugeridos son "adelantando" o "completada".

```bash
# Sintaxis
cargo run -- status --id <ID> --status "<NUEVO_ESTADO>"

# Ejemplo
cargo run -- status --id 1 --status "completada"
```

## 🧪 Ejecutar Pruebas

El proyecto incluye una suite de tests para asegurar que la lógica de negocio (CRUD) y el almacenamiento local funcionan correctamente. Para ejecutarlos:

```bash
cargo test
```

---
