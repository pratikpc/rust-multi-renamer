use crate::files::Files;
use std::{path::PathBuf, process::Command};
use tempfile::TempDir;

fn exec_program_and_file(name: &str, file: &str) -> std::io::Result<bool> {
    let status = Command::new(name).arg(file).status()?;
    let exit_ok = status.success();
    Ok(exit_ok)
}

fn select_editor() -> Option<String> {
    let visual = std::env::var("VISUAL");
    if let Ok(visual) = visual {
        return Some(visual);
    }
    let git_editor = std::env::var("GIT_EDITOR");
    if let Ok(git_editor) = git_editor {
        return Some(git_editor);
    }
    let editor = std::env::var("EDITOR");
    if let Ok(editor) = editor {
        return Some(editor);
    }

    if cfg!(windows) {
        // By default use Notepad
        // On Windows
        return Some("notepad.exe".to_string());
    }
    None
}
fn launch_file_in_editor(file: &str) -> std::io::Result<bool> {
    let editor = select_editor();
    let editor = editor.unwrap();
    exec_program_and_file(&editor, file)
}
fn save_to_json_file(path: &PathBuf, files: &Files) -> std::io::Result<()> {
    // Prettify JSON
    let json = serde_json::to_string_pretty(&files)?;
    // Write to files
    std::fs::write(path, json)
}
fn read_from_json_file(path: &PathBuf) -> std::io::Result<Files> {
    let buf = std::fs::read_to_string(path)?;
    let files: Files = serde_json::from_str(&buf)?;
    Ok(files)
}
pub fn extract_new_names(files: &Files) -> std::io::Result<Files> {
    // Create temp directory
    let temp_dir = TempDir::new()?;

    let temp_file_name = temp_dir.path().join("rename.json");

    let save_to_json = save_to_json_file(&temp_file_name, files);

    if save_to_json.is_ok() {
        // Open the filejson
        let launched_file = launch_file_in_editor(&temp_file_name.to_str().unwrap().to_string());
        if let Ok(launched_file) = launched_file {
            if launched_file {
                let files = read_from_json_file(&temp_file_name)?;
                return Ok(files);
            }
        }
    }
    let files = Files::new();
    Ok(files)
}
