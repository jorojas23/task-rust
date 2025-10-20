mod format;
mod task;

use clap::{Parser, Subcommand};
use format::ui::generar_tabla_tasks;
use task::crud::{add_task, change_status_task, delete_task, load_tasks, update_task};
use task::local_store::get_next_id;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Agrega una nueva tarea a la lista.
    Add {
        /// El título corto y descriptivo de la tarea.
        #[arg(short, long)]
        title: String,

        /// Una descripción más detallada de lo que hay que hacer.
        #[arg(short, long)]
        description: String,
    },
    
    /// Muestra todas las tareas guardadas en formato de tabla.
    List,

    /// Actualiza el título o la descripción de una tarea existente.
    Update {
        /// ID de la tarea a actualizar.
        #[arg(short, long)]
        id: u32,

        /// Nuevo título para la tarea. (Opcional)
        #[arg(short, long)]
        title: Option<String>,

        /// Nueva descripción para la tarea. (Opcional)
        #[arg(short, long)]
        description: Option<String>,
    },

    /// Elimina una tarea de la lista usando su ID.
    Delete {
        /// ID de la tarea a eliminar.
        #[arg(short, long)]
        id: u32,
    },

    /// Cambia el estado de una tarea a "Completada" o "adelantando".
    Status {
        /// ID de la tarea cuyo estado se quiere cambiar.
        #[arg(short, long)]
        id: u32,

        /// El estado se cambiara a "completada" o "adelantando".
        #[arg(short, long)]
        status: String,
    },
}
fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { title, description } => {
            handle_add_task(title, description);
        }
        Commands::List => {
            handle_list_tasks();
        }
        Commands::Update { id, title, description } => {
            handle_update_task(*id, title, description);
        }
        Commands::Delete { id } => {
            handle_delete_task(*id);
        }
        Commands::Status { id, status } => {
            handle_change_status(*id, status);
        }
    }    
    
}


fn handle_add_task(title: &str, description: &str) {
    // Lógica para agregar una tarea
    // ...
    match add_task(title, description) {
        Ok(_) => {
            println!("\n=============================================");
            println!("  Tarea agregada exitosamente:");
            println!("=============================================");
            println!("  Título: {}", title);
            println!("  Descripción: {}", description);
            println!("=============================================\n");
        },
        Err(e) => eprintln!("Error al agregar la tarea: {}", e),
    }
    
}

fn handle_list_tasks() {
    let tasks = load_tasks();
    if tasks.is_empty() {
        println!("\nNo hay tareas disponibles.\n");
        return;
    }
    match generar_tabla_tasks(tasks) {
        Ok(_) => {},
        Err(e) => eprintln!("\nError al generar la tabla de tareas: {}\n", e),
        
    }
}

fn handle_update_task(id: u32, title: &Option<String>, description: &Option<String>) {
    if title.is_none() && description.is_none() {
        println!("\nAdvertencia: No se proporcionaron datos para actualizar. Usa --title o --description.");
        return;
    }

    match update_task(id, title, description) {
        Ok(_) => println!("\nTarea con ID {} actualizada correctamente.\n", id),
        Err(e) => eprintln!("\nError al actualizar la tarea: {}\n", e),
    }
}

fn handle_delete_task(id: u32) {
    match delete_task(id) {
        Ok(_) => println!("\nTarea con ID {} eliminada correctamente.\n", id),
        Err(e) => eprintln!("\nError al eliminar la tarea: {}\n", e),
    }
}

fn handle_change_status(id: u32, status: &str) {
    match change_status_task(id, status) {
        Ok(_) => println!("\nEstado de la tarea con ID {} cambiado a '{}' correctamente.\n", id, status),
        Err(e) => eprintln!("\nError al cambiar el estado de la tarea: {}\n", e),
    }
}

