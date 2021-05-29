use std::collections::HashSet;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use anyhow::Error;
use clap::ArgMatches;
use fehler::throws;

use crate::utils::{self, ProcessPathOption};

#[derive(Debug)]
pub struct Request<'a> {
    path: PathBuf,
    targets: HashSet<&'a str>,
    exclude: HashSet<&'a str>,
    recurse: bool,
    skip_confirmation: bool,
    get_size: bool,
}

impl<'a> Request<'a> {
    pub fn new(matches: &'a ArgMatches) -> Request<'a> {
        let path = PathBuf::from(matches.value_of("directory").unwrap());
        let targets = matches.values_of("targets").unwrap().collect();
        let recurse = matches.is_present("recurse");
        let skip_confirmation = matches.is_present("skip_confirmation");
        let get_size = matches.is_present("size");
        let exclude = matches
            .values_of("exclude_directories")
            .unwrap_or_default()
            .collect();

        Request {
            path,
            targets,
            exclude,
            recurse,
            skip_confirmation,
            get_size,
        }
    }

    #[throws(Error)]
    pub fn handle(&self) {
        let mut total_size = 0;

        utils::process_paths(&self.path, |path| {
            let name = path.file_name().unwrap().to_str().unwrap();

            if self.targets.contains(name) && self.get_confirmation(path)? {
                if self.get_size {
                    total_size += utils::get_size(&path)?;
                }

                utils::remove_path(&path)?;
            } else if path.is_dir() && self.recurse && !self.exclude.contains(name) {
                return Ok(ProcessPathOption::Scan);
            }

            Ok(ProcessPathOption::None)
        })?;

        if self.get_size {
            println!("{} freed.", utils::convert_bytes(total_size as f64))
        }
    }

    #[throws(Error)]
    fn get_confirmation(&self, path: &Path) -> bool {
        if self.skip_confirmation {
            true
        } else {
            let mut stdout = io::stdout();
            write!(&mut stdout, "Delete file/directory {:?} [y/n]? ", &path)?;
            stdout.flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            input.trim().to_lowercase() == "y"
        }
    }
}
