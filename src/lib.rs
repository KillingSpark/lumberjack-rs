use std::fs::File;
use std::io::Write;
use std::ops::Sub;
use std::path::PathBuf;
use std::time;

extern crate chrono;

mod tests;

pub struct Conf {
    pub max_age: u64,
    pub max_size: u64,
    pub max_files: usize,
    pub log_dir: PathBuf,
    pub name_template: String,
}

pub struct Logger {
    conf: Conf,
    current_file: File,
    current_file_path: PathBuf,
}

use chrono::prelude::*;
fn timestamp() -> String {
    let formatted = Utc::now().to_rfc3339();

    formatted.to_string()
}
fn open_old_file(conf: &Conf) -> Option<(PathBuf,File)> {
    let files = match std::fs::read_dir(&conf.log_dir) {
        Ok(f) => f,
        Err(_) => return None,
    };
    let mut files: Vec<_> = files
        .map(|x| x.unwrap())
        .filter(|x| x.metadata().unwrap().is_file())
        .collect();

    files.sort_by(|l, r| l.path().cmp(&r.path()));

    if files.len() > 0 {
        let dir_entry = files.remove(files.len() - 1).path();
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&dir_entry)
            .unwrap();
        Some((dir_entry, file))
    } else {
        None
    }
}

fn open_next_file(conf: &Conf) -> std::io::Result<(PathBuf, File)> {
    let tmplt_split: Vec<_> = conf.name_template.split('.').collect();
    let mut new_file_name = String::new();
    new_file_name.push_str(tmplt_split[0]);
    new_file_name.push_str("__");
    new_file_name.push_str(&timestamp());
    new_file_name.push_str(".");
    for suffix in &tmplt_split[1..] {
        new_file_name.push_str(suffix);
    }

    let path = conf.log_dir.join(&new_file_name);
    return File::create(&path).map(|x| (path, x));
}

pub fn new(conf: Conf) -> std::io::Result<Logger> {
    let (path, file) = if let Some(f) = open_old_file(&conf) {
        f
    } else {
        open_next_file(&conf)?
    };
    Ok(Logger {
        conf: conf,
        current_file: file,
        current_file_path: path,
    })
}

fn days_to_secs(days: u64) -> u64 {
    days * 24 * 60 * 60
}

impl Conf {
    fn age_threshold(&self) -> time::SystemTime {
        time::SystemTime::now().sub(time::Duration::from_secs(days_to_secs(self.max_age)))
    }
}

impl Logger {
    fn enforce_conf(&mut self) -> std::io::Result<()> {
        if self.current_file.metadata()?.len() > self.conf.max_size {
            self.current_file.flush()?;
            self.current_file.sync_all()?;
            let (path, file) = open_next_file(&self.conf)?;
            self.current_file = file;
            self.current_file_path = path;
        }

        let time_threshold = self.conf.age_threshold();
        let files = std::fs::read_dir(&self.conf.log_dir)?;
        let mut resulting_files = Vec::new();
        for file in files {
            let file = file?;
            if file.path().eq(&self.current_file_path) {
                //dont remove the current file
                continue
            }
            if file.metadata()?.modified()? < time_threshold {
                std::fs::remove_file(file.path())?;
            } else {
                resulting_files.push(file);
            }
        }

        //drop old files first
        resulting_files.sort_by(|l, r| l.path().cmp(&r.path()));
        resulting_files.reverse();
        if resulting_files.len() > self.conf.max_files {
            let files_to_remove = resulting_files.len() - self.conf.max_files;
            for file in &resulting_files[0..files_to_remove] {
                std::fs::remove_file(file.path())?;
            }
        }

        Ok(())
    }
}

impl Write for Logger {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        self.enforce_conf()?;
        self.current_file.write(data)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.current_file.flush()
    }
}
