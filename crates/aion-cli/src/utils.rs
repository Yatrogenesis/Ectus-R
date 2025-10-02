// CLI Utilities

use anyhow::Result;
use dialoguer::Confirm;
use std::io::{self, Read, Write};
use std::path::Path;

/// Read multi-line input from user
pub fn read_multiline_input() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Ask for user confirmation
pub fn confirm(message: &str) -> Result<bool> {
    Ok(Confirm::new()
        .with_prompt(message)
        .default(false)
        .interact()?)
}

/// Truncate text to specified length
pub fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len - 3])
    }
}

/// Format timestamp for display
pub fn format_timestamp(timestamp: &str) -> String {
    match chrono::DateTime::parse_from_rfc3339(timestamp) {
        Ok(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        Err(_) => timestamp.to_string(),
    }
}

/// Extract ZIP archive
pub fn extract_zip(archive_path: &Path, output_dir: &Path) -> Result<()> {
    let file = std::fs::File::open(archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => output_dir.join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}