//! A simple illustration of the `cap-std` API to stores data in a project's directory

mod extension;
mod logging;

use logging::Logger;

fn main() -> anyhow::Result<()> {
    // Parse command-line arguments.
    // TODO: Show how using relative path names can lead to problems

    // Call logger from here to create appropriate dir to write
    let file_name = "dir/log.txt";
    let value = "temp";

    let logger = Logger::create_logger(file_name).unwrap();

    // Call extension from here and pass in the appropriate

    logger.append_to_log(value)?;

    // Write the value in the new file.
    // data_dir.unwrap().write(file_name, value)?;

    Ok(())
}
