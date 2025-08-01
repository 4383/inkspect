use std::fs;
use std::io::Result;
use std::path::Path;
use std::process::Command;

pub fn open_editor(file_path: &Path, editor: &str) -> Result<()> {
    Command::new(editor).arg(file_path).status()?;
    Ok(())
}

pub fn read_editor_input(file_path: &Path) -> Result<String> {
    fs::read_to_string(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_open_editor() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_editor.txt");
        let editor = "touch"; // Using `touch` as a mock editor

        let result = open_editor(&file_path, editor);
        assert!(result.is_ok());
        assert!(fs::metadata(&file_path).is_ok());
    }

    #[test]
    fn test_read_editor_input() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_read_input.txt");
        fs::write(&file_path, "test content").unwrap();

        let content = read_editor_input(&file_path).unwrap();
        assert_eq!(content, "test content");
    }
}
