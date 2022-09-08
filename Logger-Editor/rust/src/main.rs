//! A simple illustration of the `cap-std` API to stores data in a project's directory

mod extension;
mod logging;

use logging::Logger;
use extension::WordCloud;

// Show how using relative path names can lead to problems

fn main() -> anyhow::Result<()> {
    // TODO: Parse command-line arguments.

    let file_name = "dir/log.txt";
    let value = "temp";

    // Call logger from here to create appropriate dir to write
    let logger = Logger::create_logger(file_name).unwrap();

    // Call extension from here and pass in the appropriate
    let sample_word_counts = [1, 2, 3, 4, 5];

    let w_cloud = WordCloud::create_word_cloud(logger);
    w_cloud.save_total_count(&sample_word_counts);
    w_cloud.save_input(value);

    // logger.append_to_log(value)?;

    Ok(())
}
