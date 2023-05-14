/* Some imports the user may / may not need */
// use cap_directories::{ambient_authority, ProjectDirs};
// use cap_std::fs::{Dir, OpenOptions};
// use std::ffi::OsString;
// use std::io::Write;
// use std::path::PathBuf;

pub struct Logger {
    // TODO: Define the fields for logger structure
}

impl Logger {
   /// Create logger object from $DATA_DIR as the root dir and a relative file
   /// location from within the root dir
    pub fn create_logger(rel_file_name: &str) -> anyhow::Result<Self> {
        Ok(Logger {})
    }

    /// Appends text entry to logger file, or create it if it doesn't exist
    pub fn append_to_log(&self, entry: String) -> anyhow::Result<()> {
        Ok(())
    }
}
