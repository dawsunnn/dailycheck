use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use chrono::Local;

use crate::task::Task;

pub fn data_dir() -> PathBuf {
    dirs_home().join(".dailycheck")
}

fn dirs_home() -> PathBuf {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

pub fn today_file() -> PathBuf {
    let date = Local::now().format("%Y-%m-%d").to_string();
    data_dir().join(format!("{}.txt", date))
}

pub fn ensure_data_dir() -> io::Result<()> {
    let dir = data_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(())
}

pub fn load_tasks(path: &PathBuf) -> io::Result<Vec<Task>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path)?;
    Ok(content.lines().filter_map(Task::from_line).collect())
}

pub fn save_tasks(path: &PathBuf, tasks: &[Task]) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    for task in tasks {
        writeln!(file, "{}", task.to_line())?;
    }
    Ok(())
}

/// Retourne les fichiers `.txt` du dossier de données, triés du plus récent au plus ancien.
pub fn list_files() -> io::Result<Vec<PathBuf>> {
    let dir = data_dir();
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |ext| ext == "txt"))
        .collect();
    files.sort_by(|a, b| b.cmp(a));
    Ok(files)
}
