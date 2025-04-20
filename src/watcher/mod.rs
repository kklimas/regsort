use crate::config::{Config, CoreProperties};
use crate::executor::TaskExecutor;
use notify::event::CreateKind;
use notify::{recommended_watcher, Event, EventKind, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;
use tracing::{debug, info};
use walkdir::WalkDir;

pub struct CustomWatcher {
    properties: CoreProperties,
    task_executor: TaskExecutor,
}

impl CustomWatcher {
    pub fn new(config: Config) -> Self {
        let properties = config.config;
        let dry_run = properties.dry_run;
        let rules = config.rules;
        CustomWatcher {
            properties,
            task_executor: TaskExecutor::new(dry_run, rules),
        }
    }

    pub fn watch(&self) -> notify::Result<()> {
        self.run_initial_clean_up();
        self.subscribe_to_source()
    }

    fn run_initial_clean_up(&self) {
        let source_dir = &self.properties.source_dir;

        info!("Running initial clean up in {:?}", source_dir);

        for entry in WalkDir::new(source_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let file_path_buf = entry.path().to_path_buf();
            self.task_executor.execute(file_path_buf)
        }

        info!("Initial clean up finished.");

    }

    fn subscribe_to_source(&self) -> notify::Result<()> {
        let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
        let mut watcher = recommended_watcher(tx)?;

        let source_dir = &self.properties.source_dir;

        info!("Subscribing to changes in {:?}", source_dir);

        watcher.watch(Path::new(source_dir), RecursiveMode::Recursive)?;

        rx.iter()
            .map(|event| event.expect("Cannot unwrap event"))
            .filter(|event| match event.kind {
                EventKind::Create(CreateKind::File) => true,
                _ => false,
            })
            .for_each(|event| {
                debug!("Received event: {:?}", event);
                for path in event.paths {
                    if path.exists() {
                        self.task_executor.execute(path);
                    }
                }
            });
        Ok(())
    }
}
