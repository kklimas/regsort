use crate::config::Config;
use crate::utils::init_logger;
use crate::watcher::CustomWatcher;
use std::env;

mod config;
mod executor;
mod task;
mod utils;
mod watcher;

fn main() -> notify::Result<()> {
    let args: Vec<String> = env::args().collect();

    let config_file_path: String;

    if args.len() < 2 {
        let key = "REG_SORT_CONFIG";

        match env::var(key) {
            Ok(value) => {
                config_file_path = value.clone()
            },
            Err(_) => {
                eprintln!("Please specify a config file using either 'cargo run -- path/to/config.toml' or an environment variable {key}.");
                std::process::exit(1)
            },
        }
    } else {
        config_file_path = args[1].clone();
    }

    let config = Config::from_file(&config_file_path);

    init_logger(config.config.log);

    let watcher = CustomWatcher::new(config);
    watcher.watch()
}
