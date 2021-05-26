use std::collections::HashSet;
use std::f64;
use std::fs;
use std::io::{self, Write};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use clap::ArgMatches;

mod clap_app;

#[derive(Debug)]
struct Config<'a> {
    path: PathBuf,
    targets: HashSet<&'a str>,
    exclude: HashSet<&'a str>,
    recurse: bool,
    skip_confirmation: bool,
    get_size: bool,
}

#[derive(Debug)]
struct Contents {
    files: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
}

enum PathFilterOption {
    Append,
    Scan,
    Ignore,
}

fn main() {
    let matches = clap_app::app().get_matches();
    let config = build_config(&matches);
    println!("{:#?}", config);

    let del_paths = get_del_paths(&config);
    let size = handle_deletion(&config, del_paths);
    if config.get_size {
        println!("{} freed.", convert_bytes(size as f64))
    }
}

fn build_config<'a>(matches: &'a ArgMatches) -> Config<'a> {
    let path = PathBuf::from(matches.value_of("directory").unwrap());
    let targets = HashSet::from_iter(matches.values_of("targets").unwrap());
    let exclude = HashSet::from_iter(matches.values_of("exclude_directories").unwrap());
    let recurse = matches.is_present("recurse");
    let skip_confirmation = matches.is_present("skip_confirmation");
    let get_size = matches.is_present("size");

    Config {
        path,
        targets,
        exclude,
        recurse,
        skip_confirmation,
        get_size,
    }
}

fn get_dir_contents(path: &Path) -> Vec<PathBuf> {
    let entries = path.read_dir().unwrap();
    entries.map(|entry| entry.unwrap().path()).collect()
}

fn filter_paths<F>(path: &PathBuf, callback: F) -> Vec<PathBuf>
where
    F: Fn(&PathBuf) -> PathFilterOption,
{
    let mut filtered_paths: Vec<PathBuf> = Vec::new();
    let mut dirs: Vec<PathBuf> = vec![path.clone()];

    while let Some(dir) = dirs.get(0) {
        for path in get_dir_contents(dir) {
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

fn get_del_paths(config: &Config) -> Vec<PathBuf> {
    filter_paths(&config.path, &|path: &PathBuf| -> PathFilterOption {
        let name = path.file_name().unwrap().to_str().unwrap();

        if config.targets.contains(name) {
            PathFilterOption::Append
        } else if path.is_dir() && config.recurse && !config.exclude.contains(name) {
            PathFilterOption::Scan
        } else {
            PathFilterOption::Ignore
        }
    })
}

fn handle_deletion(config: &Config, paths: Vec<PathBuf>) -> u64 {
    let mut total_size = 0;

    for path in paths {
        if config.skip_confirmation {
            total_size += if config.get_size { get_size(&path) } else { 0 };
            remove_path(&path);
        } else if get_confirmation(&path) {
            total_size += if config.get_size { get_size(&path) } else { 0 };
            remove_path(&path);
        } else {
            println!("Skipping file/directory {:?}", &path);
        }
    }

    total_size
}

fn remove_path(path: &PathBuf) {
    if path.is_file() {
        fs::remove_file(&path).unwrap();
    } else if path.is_dir() {
        fs::remove_dir_all(&path).unwrap();
    }
}

fn get_all_files(path: &PathBuf) -> Vec<PathBuf> {
    if path.is_file() {
        vec![path.clone()]
    } else {
        filter_paths(path, |path| -> PathFilterOption {
            if path.is_dir() {
                PathFilterOption::Scan
            } else {
                PathFilterOption::Append
            }
        })
    }
}

fn get_size(path: &PathBuf) -> u64 {
    get_all_files(path)
        .iter()
        .fold(0, |acc, path| acc + fs::metadata(path).unwrap().len())
}

fn convert_bytes(bytes: f64) -> String {
    const SIZES: [&str; 5] = ["Bytes", "KB", "MB", "GB", "TB"];

    if bytes == 0f64 {
        "0 Bytes".to_owned()
    } else {
        let num = bytes.log(f64::consts::E) / 1024f64.log(f64::consts::E);
        let i = num.trunc() as i32;
        format!("{:.1} {}", (bytes / 1024f64.powi(i)), SIZES[i as usize])
    }
}

fn get_confirmation(path: &PathBuf) -> bool {
    let mut stdout = io::stdout();
    write!(&mut stdout, "Delete file/directory {:?} [y/n]? ", &path);
    stdout.flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_lowercase() == "y"
}
