use std::f64;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Error;
use fehler::throws;

pub enum PathFilterOption {
    Append,
    Scan,
    Ignore,
}

#[throws(Error)]
pub fn filter_paths<F>(path: &Path, mut callback: F) -> Vec<PathBuf>
where
    F: FnMut(&Path) -> PathFilterOption,
{
    let mut filtered_paths: Vec<PathBuf> = Vec::new();
    let mut dirs: Vec<PathBuf> = vec![path.to_path_buf()];

    while let Some(dir) = dirs.get(0) {
        for path in get_dir_contents(dir)? {
            match callback(&path) {
                PathFilterOption::Append => filtered_paths.push(path),
                PathFilterOption::Scan => dirs.push(path),
                PathFilterOption::Ignore => (),
            }
        }

        dirs.remove(0);
    }

    filtered_paths
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

        filter_paths(path, |path: &Path| -> PathFilterOption {
            if path.is_dir() {
                PathFilterOption::Scan
            } else {
                size += fs::metadata(path).unwrap().len();
                PathFilterOption::Ignore
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

#[throws(Error)]
fn get_dir_contents(path: &Path) -> Vec<PathBuf> {
    let entries = path.read_dir()?;
    entries.map(|entry| entry.unwrap().path()).collect()
}
