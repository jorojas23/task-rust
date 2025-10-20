use std::fs;
use std::io::{self, ErrorKind};


const FILE_PATH: &str = "src/ID.id"; 


fn read_current_id() -> io::Result<String> {

    fs::read_to_string(FILE_PATH)
}


pub fn get_next_id() -> io::Result<u32> {
 
    let current_id_str = match read_current_id() {
   
        Ok(s) => s, 
        
        
        Err(e) if e.kind() == ErrorKind::NotFound => {
            println!("Archivo de ID no encontrado. Creando con ID inicial '0'.");
            fs::write(FILE_PATH, "0")?; // Si falla la escritura, el '?' propaga el error
            "0".to_string()
        },
        
      
        Err(e) => return Err(e),
    };

    let mut next_id = 1;
    if !current_id_str.trim().is_empty() {
        let current_id = current_id_str.trim().parse::<u32>().unwrap_or_else(|_| {
            eprintln!("ADVERTENCIA: Contenido inválido en ID.id. Reiniciando ID a 0.");
            0
        });
        next_id = current_id + 1;
    }

    fs::write(FILE_PATH, next_id.to_string())?;

    Ok(next_id)
}

