use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use tempfile::Builder;

fn setup_test_environment() -> (tempfile::TempDir, PathBuf) {
    let temp_dir = Builder::new()
        .prefix("test_id")
        .tempdir()
        .expect("Fallo al crear tempdir");
    let file_path = temp_dir.path().join("test_id.id");
    (temp_dir, file_path)
}

#[test]
fn test_initial_id_creation() -> io::Result<()> {
    // _temp_dir se mantiene para que el directorio no se borre hasta el final del test
    let (_temp_dir, file_path) = setup_test_environment();

    assert!(!file_path.exists());

    let first_id = get_next_id_for_test(&file_path)?;
    assert_eq!(first_id, 1);

    assert_eq!(fs::read_to_string(&file_path)?, "1");

    Ok(())
}

#[test]
fn test_sequential_id_increment() -> io::Result<()> {
    let (_temp_dir, file_path) = setup_test_environment();

    fs::write(&file_path, "5")?;

    assert_eq!(get_next_id_for_test(&file_path)?, 6);
    assert_eq!(fs::read_to_string(&file_path)?, "6");

    assert_eq!(get_next_id_for_test(&file_path)?, 7);
    assert_eq!(fs::read_to_string(&file_path)?, "7");

    Ok(())
}

fn read_current_id_for_test(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

fn get_next_id_for_test(path: &Path) -> io::Result<u32> {
    let current_id_str = match read_current_id_for_test(path) {
        Ok(s) => s,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            fs::write(path, "0")?;
            "0".to_string()
        }
        Err(e) => return Err(e),
    };

    let current_id = current_id_str.trim().parse::<u32>().unwrap_or(0);
    let next_id = current_id + 1;

    fs::write(path, next_id.to_string())?;

    Ok(next_id)
}
