use crate::config::MatchingRegexRule;
use crate::task::Task;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

pub struct TaskExecutor {
    dry_run: bool,
    rules: Vec<MatchingRegexRule>,
}

impl TaskExecutor {
    pub fn new(dry_run: bool, rules: Vec<MatchingRegexRule>) -> Self {
        Self { dry_run, rules }
    }

    pub fn execute(&self, path_buf: PathBuf) {
        info!("Checking file from: {:?}", path_buf);

        let path = Path::new(&path_buf);

        let file_name = path.file_name().unwrap().to_str().unwrap();

        // run matching
        let matching_rule = self.find_matching_rule(file_name);

        match matching_rule {
            Some(matching_rule) => {
                // run moving
                info!(
                    "Found matching rule for file: '{}' ('{}')",
                    file_name, matching_rule.regex
                );
                let task = Task::new(
                    path.to_string_lossy().to_string(),
                    file_name.to_string(),
                    matching_rule.target.clone(),
                    self.dry_run,
                );
                task.execute();
            }
            None => {
                warn!("No matching rule found for: '{}'", file_name);
            }
        }
    }

    fn find_matching_rule(&self, file_name: &str) -> Option<&MatchingRegexRule> {
        self.rules
            .iter()
            .find(|&rule| rule.regex.is_match(file_name))
    }
}
