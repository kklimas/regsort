use std::fs::{create_dir_all, rename};
use std::io;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info};

#[derive(Clone)]
pub struct Task {
    pub full_path: String,
    pub file_name: String,
    pub target_dir: String,
    pub dry_run: bool,
}

impl Task {
    pub fn new(full_path: String, file_name: String, target_dir: String, dry_run: bool) -> Task {
        Task {
            full_path,
            file_name,
            target_dir,
            dry_run,
        }
    }
}

impl Task {
    pub fn execute(&self) {
        let target_path = self.find_target_path(None);

        info!(
            "Moving file '{}': '{}' ---> '{:?}'",
            self.file_name,
            self.full_path,
            &target_path
        );

        if !self.dry_run {

            match self.move_file(Path::new(&target_path)) {
                Ok(_) => {
                    info!("File '{}' moved from '{}' to '{:?}'", self.file_name, self.full_path, &target_path);
                },
                Err(e) => {
                    error!("Error moving file '{}': '{}'", self.file_name, e);
                }
            }
        }
    }

    fn find_target_path(&self, index: Option<i32>) -> PathBuf {
        // check if target path is taken
        debug!("Searching target path for: '{}'", &self.file_name);
        let target_path = match index {
            Some(index) => {
                let extended_file_name = &Self::extend_file(self.file_name.clone(), index);
                PathBuf::from(&self.target_dir).join(extended_file_name)
            },
            None => {
                PathBuf::from(&self.target_dir).join(&self.file_name)
            },
        };

        if target_path.exists() {
            debug!("Target path '{}' already taken", target_path.display());
            let index = match index {
                Some(index) => Some(index + 1),
                None => Some(1),
            };
            return self.find_target_path(index);
        }


        debug!("Target path '{}' is free", target_path.display());
        target_path
    }

    fn extend_file(file_name: String, id: i32) -> String {
        match file_name.rfind('.') {
            Some(pos) => {
                let (name, extension) = file_name.split_at(pos);
                format!("{}({}){}", name, id, extension)
            }
            None => format!("{}({})", file_name, id),
        }
    }

    fn move_file(&self, dst: &Path) -> io::Result<()> {
        let src = Path::new(&self.full_path);
        if let Some(parent) = dst.parent() {
            create_dir_all(parent)?;
        }
        rename(src, dst)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::task::Task;
    use serial_test::serial;
    use std::fs::{File, remove_file};
    use std::io::Write;
    use std::path::Path;

    #[test]
    #[serial]
    fn test_execute_task() {
        // given
        let target_file_path = "tests/resources/task/output/test.txt";
        let file_path = "tests/resources/task/input/test.txt";
        let target_dir = "tests/resources/task/output";
        let file_name = "test.txt";
        let dry_run = false;

        create_file(file_path);

        let task = Task::new(
            file_path.to_string(),
            file_name.to_string(),
            target_dir.to_string(),
            dry_run,
        );

        // when
        task.execute();

        // then
        let target_path = Path::new(target_file_path);
        assert!(target_path.exists());
        assert!(target_path.is_file());

        delete_file(target_file_path)
    }

    #[test]
    #[serial]
    fn test_execute_task_dry_run() {
        // given
        let file_path = "tests/resources/task/input/test.txt";
        let target_dir = "tests/resources/task/output";
        let file_name = "test.txt";
        let dry_run = true;

        create_file(file_path);

        let task = Task::new(
            file_path.to_string(),
            file_name.to_string(),
            target_dir.to_string(),
            dry_run,
        );

        // when
        task.execute();

        // then
        let target_path = Path::new(file_path);
        assert!(target_path.exists());
        assert!(target_path.is_file());

        delete_file(file_path)
    }

    fn create_file(file_path: &str) {
        let mut file = File::create(file_path).unwrap();
        file.write_all(b"Rust is super cool!!!").unwrap();
    }

    fn delete_file(file_path: &str) {
        remove_file(file_path).expect("Could not delete file");
    }
}
