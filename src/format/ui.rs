use cli_table::{
    format::{Separator},
    print_stdout,
    Table,
}; 
use crate::task::crud::Task;


pub fn generar_tabla_tasks(tasks: Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Lista de Tareas ---");

    let tabla = tasks
        .table()
        
        .title([
            "ID", 
            "Título", 
            "Descripción", 
            "F. Inicio", 
            "F. Fin", 
            "Estado"
        ])

        .separator(Separator::builder().build());

    print_stdout(tabla)?;

    Ok(())
}
