// NOTA: Para que este test de integración funcione, la crate (o el proyecto) debe
// exponer públicamente (pub use) Task, Status, get_next_id, get_current_date, etc.

use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
// Importamos todo lo que necesitamos asumiendo que está expuesto desde el crate
// El prefijo 'crate_name::' se reemplaza con el nombre real de tu proyecto si es una librería.

// Aquí deberías cambiar 'su_crate' por el nombre real de tu proyecto en Cargo.toml
// Por simplicidad, asumiremos que las funciones mock están definidas aquí para ser autocontenidas.


// --- Funciones de Utilidad y Mocks ---

// Necesitas 'Status' y 'Task' aquí para que el código compile.
// Si tu proyecto no expone estas structs, debes copiarlas aquí.
#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Status {
    Pendiente,
    Adelantando,
    Hecha,
}

impl std::str::FromStr for Status {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pendiente" => Ok(Status::Pendiente),
            "adelantando" => Ok(Status::Adelantando),
            "hecha" => Ok(Status::Hecha),
            _ => Err(()),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub f_start: String,
    pub f_end: Option<String>,
}


/// Mock de get_next_id: Simplemente encuentra el ID más alto y suma 1.
fn get_next_id(tasks: &[Task]) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

/// Mock de get_current_date: Devuelve una fecha simulada.
fn get_current_date() -> String {
    "2025-10-19 12:00:00".to_string()
}

/// Crea un archivo temporal para un test y devuelve su path.
fn setup_test_file() -> (tempfile::TempDir, PathBuf) {
    let temp_dir = tempfile::tempdir().unwrap();
    // Solución compatible y limpia: Primero creamos el nombre del archivo.
    let file_name = format!("{}.json", Uuid::new_v4());
    let file_path = temp_dir.path().join(&file_name);
    (temp_dir, file_path)
}

// --- Funciones Mock que operan sobre un path específico ---

fn load_tasks_mock(path: &Path) -> Result<Vec<Task>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(path)?;
    let tasks: Vec<Task> = serde_json::from_str(&data)?;
    Ok(tasks)
}

fn save_tasks_mock(tasks: &[Task], path: &Path) -> Result<()> {
    let data = serde_json::to_string_pretty(tasks)?;
    fs::write(path, data)?;
    Ok(())
}

fn add_task_mock(title: &str, description: &str, path: &Path) -> Result<()> {
    let mut tasks = load_tasks_mock(path).unwrap_or_else(|_| Vec::new());
    let new_task = Task {
        id: get_next_id(&tasks),
        title: title.to_string(),
        description: description.to_string(),
        status: Status::Pendiente,
        f_start: get_current_date(),
        f_end: None,
    };
    tasks.push(new_task);
    save_tasks_mock(&tasks, path)
}

fn update_task_mock(id: u32, title: &Option<String>, description: &Option<String>, path: &Path) -> Result<()> {
    let mut tasks = load_tasks_mock(path)?;
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        if let Some(new_title) = title {
            task.title = new_title.clone();
        }
        if let Some(new_desc) = description {
            task.description = new_desc.clone();
        }
    }
    save_tasks_mock(&tasks, path)
}

fn delete_task_mock(id: u32, path: &Path) -> Result<bool> {
    let mut tasks = load_tasks_mock(path)?;
    let initial_len = tasks.len();
    tasks.retain(|t| t.id != id);
    if tasks.len() < initial_len {
        save_tasks_mock(&tasks, path)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn change_status_task_mock(id: u32, status_str: &str, path: &Path) -> Result<bool> {
    let mut tasks = load_tasks_mock(path)?;
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        if let Ok(new_status) = status_str.parse::<Status>() {
            task.status = new_status;
            if task.status == Status::Hecha {
                task.f_end = Some(get_current_date());
            }
            save_tasks_mock(&tasks, path)?;
            return Ok(true);
        }
    }
    Ok(false)
}

// --- TESTS ---

#[test]
fn test_add_task() -> Result<()> {
    let (_temp_dir, path) = setup_test_file();
    add_task_mock("Comprar leche", "Tienda", &path)?;
    add_task_mock("Estudiar Rust", "Tests", &path)?;

    let tasks = load_tasks_mock(&path)?;
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[1].id, 2);
    assert_eq!(tasks[0].title, "Comprar leche");

    Ok(())
}

#[test]
fn test_update_task() -> Result<()> {
    let (_temp_dir, path) = setup_test_file();
    add_task_mock("Título Antiguo", "Desc Antigua", &path)?;
    
    let new_title = Some("Título Nuevo".to_string());
    update_task_mock(1, &new_title, &None, &path)?;
    let tasks_after_title_update = load_tasks_mock(&path)?;
    assert_eq!(tasks_after_title_update[0].title, "Título Nuevo");
    
    let new_desc = Some("Desc Nueva".to_string());
    update_task_mock(1, &None, &new_desc, &path)?;
    let tasks_after_desc_update = load_tasks_mock(&path)?;
    assert_eq!(tasks_after_desc_update[0].description, "Desc Nueva");

    Ok(())
}

#[test]
fn test_delete_task_success() -> Result<()> {
    let (_temp_dir, path) = setup_test_file();
    add_task_mock("T1", "D1", &path)?;
    add_task_mock("T2", "D2", &path)?;
    
    assert!(delete_task_mock(1, &path)?);
    
    let tasks = load_tasks_mock(&path)?;
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, 2);
    
    Ok(())
}

#[test]
fn test_delete_task_not_found() -> Result<()> {
    let (_temp_dir, path) = setup_test_file();
    add_task_mock("T1", "D1", &path)?;
    
    // Debería devolver 'false' si no encuentra el ID
    assert!(!delete_task_mock(99, &path)?); 
    
    let tasks = load_tasks_mock(&path)?;
    assert_eq!(tasks.len(), 1); // La lista no debe cambiar

    Ok(())
}

#[test]
fn test_change_status_task_to_completed() -> Result<()> {
    let (_temp_dir, path) = setup_test_file();
    add_task_mock("T1", "D1", &path)?;

    assert!(change_status_task_mock(1, "hecha", &path)?);
    
    let tasks = load_tasks_mock(&path)?;
    assert_eq!(tasks[0].status, Status::Hecha);
    assert!(tasks[0].f_end.is_some()); // Debe tener fecha de fin

    Ok(())
}
