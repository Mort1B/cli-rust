use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Taks {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

pub fn add_task(jounal_path: PathBuf, task: Task) -> Result<()> {
    //Open the file
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(journal_path)?;

    let mut tasks: Vec<Task> = match serde_json::from_reader(&file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    // rewind the file after reading form it.
    file.seek(SeekFrom::Start(0))?;

    //Write modified dtask list back into the file
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn complete_task(jounal_path: PathBuf, task_position: usize) -> Result<()> {
    // open file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(jounal_path)?;

    //consume firel's contents as vector of tasks
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid task ID"));
    }
    tasks.remove(task_position - 1);

    //Rewind and truncate the file
    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;

    //Write the modified task list back into the file
    serde_json::to_writer(file, &tasks)?;
}
