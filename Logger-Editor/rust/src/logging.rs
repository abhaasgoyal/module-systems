use cap_directories::{ambient_authority, ProjectDirs};
use cap_std::fs::Dir;
use std::ffi::OsString;
use std::path::PathBuf;

pub struct Logger {
    data_dir: Dir,
    file_name: OsString,
}

impl Logger {
    pub fn create_logger(rel_file_name: &str) -> anyhow::Result<Self> {
        let key: PathBuf = PathBuf::from(rel_file_name);

        // Obtain the `data_dir` for this program.
        let project_dirs = ProjectDirs::from(
            "com.example",
            "Example Organization",
            "Cap-std Key-Value CLI Example",
            ambient_authority(),
        )
        .unwrap();
        let mut data_dir = project_dirs.data_dir()?;

        // Create new directory and set data_dir to that path
        let parent = key.parent();
        let file_name = key.file_name().unwrap().to_owned();
        // let file_name = key.file_name().unwrap();
        if let Some(parent) = parent {
            if !parent.as_os_str().is_empty() {
                data_dir.create_dir_all(parent)?;
                data_dir = data_dir.open_dir(parent)?;
            }
        }

        Ok(Logger {
            data_dir,
            file_name,
        })
    }

    pub fn append_to_log(&self, entry: String) -> anyhow::Result<()> {
        let Logger {
            data_dir,
            file_name,
        } = self;
        data_dir.write(file_name, entry)?;
        Ok(())
    }
}
