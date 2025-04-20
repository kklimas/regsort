use regex::Regex;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    pub config: CoreProperties,
    pub rules: Vec<MatchingRegexRule>,
}

impl Config {
    pub fn from_file(file_path: &str) -> Self {
        let file_config = parse_config_from_file(file_path).expect("Could not parse config file");

        let source_dir_path = &file_config.config.source_dir;
        let source_dir = Path::new(source_dir_path);

        if !source_dir.is_dir() {
            panic!("The source dir is not a directory");
        }

        let rules = file_config
            .rules
            .iter()
            .map(|rule| MatchingRegexRule::new(rule))
            .collect::<Vec<MatchingRegexRule>>();

        Self {
            config: file_config.config,
            rules,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct FileConfig {
    pub config: CoreProperties,
    pub rules: Vec<MatchingRule>,
}

#[derive(Deserialize, Debug)]
pub struct CoreProperties {
    pub source_dir: String,
    pub log: bool,
    pub dry_run: bool,
}

#[derive(Deserialize, Debug)]
pub struct MatchingRule {
    pub pattern: String,
    pub target: String,
}

#[derive(Debug)]
pub struct MatchingRegexRule {
    pub regex: Regex,
    pub target: String,
}

impl MatchingRegexRule {
    pub fn new(rule: &MatchingRule) -> Self {
        let pattern = rule.pattern.as_str();
        let target = rule.target.clone();

        let regex = Regex::new(pattern).unwrap();
        Self {
            regex,
            target,
        }
    }
}

fn parse_config_from_file(file_path: &str) -> Result<FileConfig, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let config: FileConfig = toml::de::from_str(&contents)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn test_parse_config_from_file() {
        // given
        let config_file_path = "tests/resources/config/config.toml";

        // when
        let parsed_config = Config::from_file(config_file_path);

        // then
        let core_config = &parsed_config.config;
        assert_eq!(core_config.source_dir, "tests/resources/config/temp/input".to_string());
        assert_eq!(core_config.dry_run, false);
        assert_eq!(core_config.log, true);

        let rules = &parsed_config.rules;
        assert_eq!(rules.len(), 2);

        let first_rule = &rules[0];
        assert!(first_rule.regex.is_match("file.txt"));
        assert!(first_rule.regex.is_match("file123.txt"));
        assert_eq!(first_rule.target, "tests/resources/config/temp/output/txt");

        let second_rule = &rules[1];
        assert!(second_rule.regex.is_match("file.csv"));
        assert!(second_rule.regex.is_match("file123.csv"));
        assert_eq!(second_rule.target, "tests/resources/config/temp/output/csv");
    }

    #[test]
    #[should_panic(expected = "Could not parse config file")]
    fn test_parse_config_from_file_source_unable_to_parse() {
        // given
        let config_file_path = "tests/resources/config/config_invalid_struct.toml";

        // when then
        Config::from_file(config_file_path);
    }

    #[test]
    #[should_panic(expected = "The source dir is not a directory")]
    fn test_parse_config_from_file_source_dir_not_found() {
        // given
        let config_file_path = "tests/resources/config/config_no_source_dir.toml";

        // when then
        Config::from_file(config_file_path);
    }
}
