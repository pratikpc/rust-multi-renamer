use std::fs;

use crate::files::Files;

pub fn rename(source: &Files, dest: &Files) -> std::io::Result<()> {
    for (dest_idx, dest_name) in dest.into_iter() {
        let source_name = source.get(dest_idx);
        if let Some(source_name) = source_name {
            if source_name != dest_name {
                fs::rename(source_name, dest_name)?;
            }
        }
    }
    Ok(())
}
