use cap_directories::{ambient_authority, ProjectDirs};
use std::path::PathBuf;
use cap_std::fs::Dir;

pub fn create_logger(rel_file_name: &str, value: &str) -> anyhow::Result<Dir> {
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
    let file_name = key.file_name().unwrap();
    if let Some(parent) = parent {
        if !parent.as_os_str().is_empty() {
            data_dir.create_dir_all(parent)?;
            data_dir = data_dir.open_dir(parent)?;
        }
    }

    Ok(data_dir)
}

/*
pub fn save_log(data_dir: Result<Dir>, value: String) -> () {
    data_dir.write("log.txt", value)?;
}

*/
