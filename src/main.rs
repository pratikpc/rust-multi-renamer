use crate::{
    extract_new_names::extract_new_names,
    io::{reader, rename},
};

mod extract_new_names;
mod files;
mod io;

fn pattern() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    let pattern = args.first()?;
    let pattern = pattern.clone();
    Some(pattern)
}

fn main() {
    if let Some(pattern) = pattern() {
        if let Ok(original_names) = reader(&pattern) {
            if let Ok(new_names) = extract_new_names(&original_names) {
                if !new_names.is_empty() {
                    rename(&original_names, &new_names).unwrap();
                }
            }
        }
    }
}
