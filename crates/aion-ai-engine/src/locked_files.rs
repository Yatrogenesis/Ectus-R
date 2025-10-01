// Locked Files Protection System
// Prevents autocorrection from overwriting manually edited files
// Uses git diff detection to identify human-modified files

use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{debug, info, warn};

/// Tracks files that should not be automatically modified
pub struct LockedFilesManager {
    /// Files explicitly marked as locked by user
    user_locked: HashSet<PathBuf>,

    /// Files detected as manually edited via git diff
    git_detected_locked: HashSet<PathBuf>,

    /// Project root directory
    project_root: PathBuf,
}

impl LockedFilesManager {
    pub fn new(project_root: impl Into<PathBuf>) -> Self {
        Self {
            user_locked: HashSet::new(),
            git_detected_locked: HashSet::new(),
            project_root: project_root.into(),
        }
    }

    /// Lock a file explicitly (user request)
    pub fn lock_file(&mut self, path: impl Into<PathBuf>) {
        let path = path.into();
        info!("🔒 Locking file: {:?}", path);
        self.user_locked.insert(path);
    }

    /// Unlock a file explicitly
    pub fn unlock_file(&mut self, path: &Path) {
        if self.user_locked.remove(path) {
            info!("🔓 Unlocking file: {:?}", path);
        }
        self.git_detected_locked.remove(path);
    }

    /// Check if file is locked (either explicitly or via git detection)
    pub fn is_locked(&self, path: &Path) -> bool {
        self.user_locked.contains(path) || self.git_detected_locked.contains(path)
    }

    /// Scan git diff to detect manually edited files
    /// Files with uncommitted changes are considered locked
    pub fn scan_git_modifications(&mut self) -> Result<()> {
        debug!("🔍 Scanning for git modifications...");

        // Get uncommitted changes
        let output = Command::new("git")
            .current_dir(&self.project_root)
            .args(&["diff", "--name-only"])
            .output()
            .context("Failed to run git diff")?;

        if !output.status.success() {
            warn!("Git diff failed, skipping locked file detection");
            return Ok(());
        }

        let modified_files = String::from_utf8_lossy(&output.stdout);
        let mut new_locked = HashSet::new();

        for file in modified_files.lines() {
            let path = self.project_root.join(file);
            if path.exists() {
                debug!("  📝 Detected manual edit: {:?}", path);
                new_locked.insert(path);
            }
        }

        // Also check staged changes
        let staged_output = Command::new("git")
            .current_dir(&self.project_root)
            .args(&["diff", "--cached", "--name-only"])
            .output()
            .context("Failed to run git diff --cached")?;

        if staged_output.status.success() {
            let staged_files = String::from_utf8_lossy(&staged_output.stdout);
            for file in staged_files.lines() {
                let path = self.project_root.join(file);
                if path.exists() {
                    debug!("  📝 Detected staged edit: {:?}", path);
                    new_locked.insert(path);
                }
            }
        }

        let count = new_locked.len();
        self.git_detected_locked = new_locked;

        if count > 0 {
            info!("🔒 Detected {} manually edited files (auto-locked)", count);
        }

        Ok(())
    }

    /// Get list of all locked files
    pub fn get_locked_files(&self) -> Vec<&PathBuf> {
        self.user_locked
            .iter()
            .chain(self.git_detected_locked.iter())
            .collect()
    }

    /// Load locked files from config file (.aion-locked)
    pub fn load_from_config(&mut self) -> Result<()> {
        let config_path = self.project_root.join(".aion-locked");

        if !config_path.exists() {
            debug!("No .aion-locked file found, skipping");
            return Ok(());
        }

        let content = std::fs::read_to_string(&config_path)
            .context("Failed to read .aion-locked")?;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let path = self.project_root.join(line);
            self.user_locked.insert(path);
        }

        info!("📄 Loaded {} locked files from config", self.user_locked.len());
        Ok(())
    }

    /// Save locked files to config file
    pub fn save_to_config(&self) -> Result<()> {
        let config_path = self.project_root.join(".aion-locked");

        let mut content = String::from("# AION-R Locked Files\n");
        content.push_str("# Files listed here will never be auto-modified\n");
        content.push_str("# One file path per line (relative to project root)\n\n");

        for path in &self.user_locked {
            if let Ok(rel_path) = path.strip_prefix(&self.project_root) {
                content.push_str(&format!("{}\n", rel_path.display()));
            }
        }

        std::fs::write(&config_path, content)
            .context("Failed to write .aion-locked")?;

        info!("💾 Saved {} locked files to config", self.user_locked.len());
        Ok(())
    }

    /// Check if autocorrection should be allowed on a file
    /// Returns an error with explanation if file is locked
    pub fn check_autocorrection_allowed(&self, path: &Path) -> Result<()> {
        if self.user_locked.contains(path) {
            anyhow::bail!(
                "File is explicitly locked by user: {:?}\n\
                 To allow autocorrection, run: aion unlock {:?}",
                path, path
            );
        }

        if self.git_detected_locked.contains(path) {
            anyhow::bail!(
                "File has uncommitted manual changes: {:?}\n\
                 Commit your changes or run: aion unlock {:?}",
                path, path
            );
        }

        Ok(())
    }

    /// Apply a safe merge strategy for locked files
    /// Instead of overwriting, create a .aion-suggested file
    pub fn create_suggestion_file(&self, path: &Path, suggested_content: &str) -> Result<PathBuf> {
        let mut suggestion_path = path.to_path_buf();

        // Add .aion-suggested extension
        let mut filename = suggestion_path
            .file_name()
            .context("Invalid file path")?
            .to_os_string();
        filename.push(".aion-suggested");
        suggestion_path.set_file_name(filename);

        std::fs::write(&suggestion_path, suggested_content)
            .context("Failed to write suggestion file")?;

        info!("💡 Created suggestion file: {:?}", suggestion_path);
        info!("   Review changes and merge manually if needed");

        Ok(suggestion_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_lock_unlock() {
        let temp = TempDir::new().unwrap();
        let mut manager = LockedFilesManager::new(temp.path());

        let file_path = temp.path().join("test.rs");
        manager.lock_file(&file_path);

        assert!(manager.is_locked(&file_path));
        assert_eq!(manager.get_locked_files().len(), 1);

        manager.unlock_file(&file_path);
        assert!(!manager.is_locked(&file_path));
    }

    #[test]
    fn test_config_save_load() {
        let temp = TempDir::new().unwrap();
        let mut manager = LockedFilesManager::new(temp.path());

        let file1 = temp.path().join("src/main.rs");
        let file2 = temp.path().join("tests/integration.rs");

        fs::create_dir_all(file1.parent().unwrap()).unwrap();
        fs::create_dir_all(file2.parent().unwrap()).unwrap();
        fs::write(&file1, "// test").unwrap();
        fs::write(&file2, "// test").unwrap();

        manager.lock_file(&file1);
        manager.lock_file(&file2);
        manager.save_to_config().unwrap();

        // Load in new instance
        let mut manager2 = LockedFilesManager::new(temp.path());
        manager2.load_from_config().unwrap();

        assert!(manager2.is_locked(&file1));
        assert!(manager2.is_locked(&file2));
    }

    #[test]
    fn test_check_autocorrection_allowed() {
        let temp = TempDir::new().unwrap();
        let mut manager = LockedFilesManager::new(temp.path());

        let file_path = temp.path().join("locked.rs");
        manager.lock_file(&file_path);

        let result = manager.check_autocorrection_allowed(&file_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("explicitly locked"));
    }

    #[test]
    fn test_suggestion_file() {
        let temp = TempDir::new().unwrap();
        let manager = LockedFilesManager::new(temp.path());

        let original = temp.path().join("test.rs");
        fs::write(&original, "// original").unwrap();

        let suggestion_path = manager
            .create_suggestion_file(&original, "// suggested fix")
            .unwrap();

        assert!(suggestion_path.exists());
        assert!(suggestion_path.to_string_lossy().contains(".aion-suggested"));

        let content = fs::read_to_string(suggestion_path).unwrap();
        assert_eq!(content, "// suggested fix");
    }
}
