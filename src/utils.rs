use std::collections::VecDeque;
use std::f64;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Error, Result};
use fehler::throws;

pub enum ProcessPathOption {
    Scan,
    None,
}

#[throws(Error)]
pub fn process_paths<F>(path: &Path, mut callback: F)
where
    F: FnMut(&Path) -> Result<ProcessPathOption>,
{
    let mut dirs: VecDeque<PathBuf> = VecDeque::new();
    dirs.push_back(path.to_path_buf());

    while let Some(dir) = dirs.pop_front() {
        for entry in dir.read_dir()? {
            let entry_path = entry?.path();

            if let ProcessPathOption::Scan = callback(&entry_path)? {
                dirs.push_back(entry_path);
            }
        }
    }
}

#[throws(Error)]
pub fn remove_path(path: &Path) {
    if path.is_file() {
        fs::remove_file(&path)?;
    } else if path.is_dir() {
        fs::remove_dir_all(&path)?;
    }
}

#[throws(Error)]
pub fn get_size(path: &Path) -> u64 {
    if path.is_file() {
        fs::metadata(path)?.len()
    } else {
        let mut size: u64 = 0;

        process_paths(path, |path| {
            if path.is_dir() {
                Ok(ProcessPathOption::Scan)
            } else {
                size += fs::metadata(path)?.len();
                Ok(ProcessPathOption::None)
            }
        })?;

        size
    }
}

pub fn convert_bytes(bytes: f64) -> String {
    const SIZES: [&str; 5] = ["Bytes", "KB", "MB", "GB", "TB"];

    if bytes == 0f64 {
        "0 Bytes".to_owned()
    } else {
        let num = bytes.log(f64::consts::E) / 1024f64.log(f64::consts::E);
        let i = num.trunc() as i32;
        format!("{:.1} {}", (bytes / 1024f64.powi(i)), SIZES[i as usize])
    }
}
