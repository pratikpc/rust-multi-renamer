use crate::files::Files;
use glob::{glob, PatternError};

pub fn reader(pattern: &str) -> Result<Files, PatternError> {
    let mut files = Files::new();
    for (id, name) in glob(pattern)?.enumerate() {
        if let Ok(name) = name {
            files.insert(id, name);
        }
    }
    Ok(files)
}
