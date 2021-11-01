use std::path::{Path, PathBuf};

use crate::error::ConfigError;

const CONFIGURATION_FILES: [&str; 5] = [
    ".lift/config.toml",
    ".lift.toml",
    ".muse/config.toml",
    ".muse.toml",
    ".muse/config",
];

const DEFAULT_IGNORES_FILE: &str = ".muse/ignoreFiles";

pub(crate) fn locate_files<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>, ConfigError> {
    let path = path.as_ref();
    if path.exists() && path.is_dir() {
        let valid_files = CONFIGURATION_FILES
            .iter()
            .map(|cf| path.join(cf))
            .filter(|cf| cf.exists() && cf.is_file())
            .collect();
        Ok(valid_files)
    } else {
        Err(ConfigError::FolderNotFound(path.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use temp_testdir::TempDir;

    #[test]
    fn it_finds_no_configs() {
        let temp = TempDir::default();
        let found_files = locate_files(temp).expect("expected temp to be a valid directory");
        let expected: Vec<PathBuf> = Vec::new();
        assert_eq!(found_files, expected);
    }

    #[test]
    fn it_finds_all_the_configs() {
        let temp = TempDir::default();

        for config in CONFIGURATION_FILES {
            let file_path = temp.join(config);
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(&parent)
                    .expect(&format!("Failed to create parent directory: {:?}", parent));
            }
            File::create(&file_path).expect(&format!("Failed to create {:?}", file_path.clone()));
        }

        let found_files = locate_files(&temp).expect("expected temp to be a valid directory");
        let expected: Vec<PathBuf> = vec![
            temp.join(".lift/config.toml"),
            temp.join(".lift.toml"),
            temp.join(".muse/config.toml"),
            temp.join(".muse.toml"),
            temp.join(".muse/config"),
        ];
        assert_eq!(found_files, expected);
    }
}
