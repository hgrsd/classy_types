use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn read_file(path: &Path) -> io::Result<String> {
    let file_contents = fs::read_to_string(path)?;
    Ok(file_contents)
}

pub fn write_file(out_dir: &str, original_path: &Path, file: &str) -> io::Result<String> {
    let mut path = PathBuf::new();
    let filename = create_file_name(original_path.file_name().unwrap().to_str().unwrap());
    path.push(out_dir);
    path.push(filename);
    fs::create_dir_all(out_dir)?;
    fs::write(&path, file)?;
    Ok(path.as_os_str().to_str().unwrap().to_string())
}

fn create_file_name(current_filename: &str) -> String {
    format!("classes-{}", current_filename)
}

#[test]
fn test_create_file_name() {
    assert_eq!(create_file_name("test-file.d.ts"), "classes-test-file.d.ts");
    assert_eq!(create_file_name("test-file.ts"), "classes-test-file.ts");
    assert_eq!(create_file_name("testfile_.d.ts"), "classes-testfile_.d.ts");
}
