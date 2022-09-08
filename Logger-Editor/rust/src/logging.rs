use cap_directories::{ambient_authority, ProjectDirs};
use cap_std::fs::{Dir, OpenOptions};
use std::ffi::OsString;
use std::io::Write;
use std::path::PathBuf;

pub struct Logger {
    log_dir: Dir,
    file_name: OsString,
}

impl Logger {
    pub fn create_logger(rel_file_name: &str) -> anyhow::Result<Self> {
        let key: PathBuf = PathBuf::from(rel_file_name);

        /* Obtain the `data_dir` for this program. */
        let project_dirs = ProjectDirs::from(
            "com.example",
            "Example Organization",
            "Cap-std Key-Value CLI Example",
            ambient_authority(),
        )
        .unwrap();

        // Get root dir from where relative paths won't work
        let root_dir = project_dirs.data_dir()?;
        let mut log_dir = project_dirs.data_dir()?; // So that log_dir is not potentially empty

        // Create new directory and set data_dir to that path
        let parent = key.parent();
        let file_name = key.file_name().unwrap().to_owned();

        if let Some(parent) = parent {
            if !parent.as_os_str().is_empty() {
                root_dir.create_dir_all(parent)?;
                log_dir = root_dir.open_dir(parent)?;
            }
        }

        Ok(Logger { log_dir, file_name })
    }

    pub fn append_to_log(&self, entry: String) -> anyhow::Result<()> {
        let Logger { log_dir, file_name } = self;
        // Open in write append mode, and create file if it doesn't exist
        let mut fd = log_dir.open_with(
            file_name,
            OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
        )?;

        fd.write_all(entry.as_ref())?;

        Ok(())
    }
}

fn invalid_create_logger() -> anyhow::Error {
    anyhow::Error::msg("couldn't create folder for log")
}
