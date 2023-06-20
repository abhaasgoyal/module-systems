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

    // Call to the following functions while ensuring the word count cannot access the underlying file structure
    w_cloud.log_total_count(&sample_word_counts)?;
    w_cloud.log_input(value)?;

    Ok(())
}
