use crate::logging::Logger;

pub struct WordCloud {
    logger: Logger,
}

impl WordCloud {
    /// WordCloud object Constructor
    pub fn create_word_cloud(logger: Logger) -> Self {
        WordCloud { logger }
    }

    pub fn log_total_count(&self, word_counts: &[i64]) -> anyhow::Result<()> {
        let total_word_count: i64 = word_counts.iter().sum();
        self.logger.append_to_log(total_word_count.to_string())?;
        Ok(())
    }

    pub fn log_input(&self, input: &str) -> anyhow::Result<()> {
        self.logger.append_to_log(input.to_string())?;
        Ok(())
    }
}
