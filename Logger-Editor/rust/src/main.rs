//! A simple illustration of the `cap-std` API to stores data in a project's directory

// Optional TODOs for refinement
// TODO: Parse as command-line arguments for faster prototyping.
// TODO: While creating logger, pass-in root folder?

mod extension;
mod logging;

use logging::Logger;
use extension::WordCloud;

fn main() -> anyhow::Result<()> {

    let file_name = "dir/log.txt";
    let value = "temp";

    // Create Logger and Extension Object
    let logger = Logger::create_logger(file_name).unwrap();
    let w_cloud = WordCloud::create_word_cloud(logger);

    // Sample value for computation and writing to logger file
    let sample_word_counts = [1, 2, 3, 4, 5];

    w_cloud.log_total_count(&sample_word_counts)?;
    w_cloud.log_input(value)?;

    Ok(())
}
