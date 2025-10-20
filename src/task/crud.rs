use std::fs;
use std::io::{self, ErrorKind, Result}; 
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use crate::get_next_id;

const FILE_PATH: &str = "src/datos.json";

#[derive(Deserialize, Serialize, Debug, Clone, cli_table::Table)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub f_start: String,
    pub f_end: String,
    pub status: String,
}

pub fn save_tasks(tasks: &[Task]) -> io::Result<()> { // Retorna Result
    let json_data = serde_json::to_string_pretty(tasks)?;
    
    match  fs::write(FILE_PATH, json_data) {
        Ok(_) => {
            Ok(())
        },
        Err(e) => return Err(e),

    }
}

pub fn load_tasks() -> Vec<Task> {
    // Si la lectura falla (NotFound), usa "[]". Si el JSON es inválido, usa vec![]
    let data = std::fs::read_to_string(FILE_PATH).unwrap_or_else(|e| {
        if e.kind() != ErrorKind::NotFound {
             eprintln!("Error al leer el archivo de tareas: {}", e);
        }
        "[]".to_string()
    });
    
    serde_json::from_str(&data).unwrap_or_else(|e| {
        eprintln!("ADVERTENCIA: Archivo de tareas corrupto. Iniciando lista vacía. Error: {}", e);
        vec![]
    })
}

pub fn add_task(title: &str, description: &str) -> Result<()> { 
    let mut tasks = load_tasks();
    
    let next_id = get_next_id()?; 
    let now_local: DateTime<Local> = Local::now();
    let formatted_local = now_local.format("%Y-%m-%d %H:%M:%S");

    let new_task = Task {
        id: next_id,
        title: title.to_string(),
        description: description.to_string(),
        f_start: formatted_local.to_string(),
        f_end: "".to_string(),
        status: "Pendiente".to_string(),
    };
    
    tasks.push(new_task);
    
   
    save_tasks(&tasks)?;
    
    println!("Tarea con ID {} agregada con éxito.", next_id);
    Ok(())
}

pub fn update_task(id: u32, title: &Option<String>, description: &Option<String>) -> Result<()> {
    let mut tasks = load_tasks();
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        if let Some(new_title) = title {
            task.title = new_title.clone();
        }
        if let Some(new_description) = description {
            task.description = new_description.clone();
        }
        save_tasks(&tasks)?; 
    } else {
        eprintln!("Advertencia: Tarea con ID {} no encontrada.", id);
    }
    Ok(())
}

// Función DELETE_TASK
pub fn delete_task(id: u32) -> Result<()> {
    let mut tasks = load_tasks();
    let initial_count = tasks.len();
    
    tasks.retain(|task| task.id != id);
    
    if tasks.len() < initial_count {
        save_tasks(&tasks)?; 
        println!("Tarea con ID {} eliminada con éxito.", id);
    } else {
        eprintln!("Advertencia: No se encontró tarea con ID {} para eliminar.", id);
    }
    Ok(())
}

pub fn change_status_task(id: u32, status: &str) -> Result<()> {
    let mut tasks = load_tasks();
    let status_lower = status.to_lowercase();
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) && (status_lower == "pendiente" || status_lower == "adelantando" || status_lower == "completada") {
        task.status = status.to_string();
        if task.status == "completada" {
            let now_local: DateTime<Local> = Local::now();
            let formatted_local = now_local.format("%Y-%m-%d %H:%M:%S");
            task.f_end = formatted_local.to_string();
        }
        save_tasks(&tasks)?; 
        println!("Estado de la tarea con ID {} cambiado a '{}'.", id, status);
    } else {
        eprintln!("Advertencia: Tarea con ID {} no encontrada para cambiar estado.", id);
    }
    Ok(())
}
