use clap::{App, Arg, ArgMatches};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::Path;
use std::path::PathBuf;

// TODO Parse the arguments. Done
// TODO Get list of directory.

#[derive(Debug)]
struct Config<'a> {
    path: PathBuf,
    targets: HashSet<&'a str>,
    exclude: HashSet<&'a str>,
    recurse: bool,
    skip_confirmation: bool,
    get_size: bool,
}

fn main() {
    let matches = App::new("del-files")
        .version("0.2")
        .about("Make notes from command line")
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("directory")
                .takes_value(true)
                .required(true)
                .help("Specify the directory to search in."),
        )
        .arg(
            Arg::with_name("targets")
                .short("t")
                .long("targets")
                .value_name("FILE")
                .takes_value(true)
                .multiple(true)
                .required(true)
                .help("Files or directories to delete."),
        )
        .arg(
            Arg::with_name("exclude_directories")
                .short("e")
                .long("exlude-directories")
                .takes_value(true)
                .multiple(true)
                .help("Specify the directory to exclude from search."),
        )
        .arg(
            Arg::with_name("skip_confirmation")
                .short("y")
                .long("skip-confirmation")
                .help("Skip confirming when deleting file/directory."),
        )
        .arg(
            Arg::with_name("recurse")
                .short("r")
                .long("recurse")
                .help("Search recursively for files/directories."),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .help("Output the disk space freed."),
        )
        .get_matches();

    let config = build_config(&matches);
    println!("{:#?}", config);
}

fn build_config<'a>(matches: &'a ArgMatches) -> Config<'a> {
    let path = PathBuf::from(matches.value_of("directory").unwrap());
    let targets = HashSet::from_iter(matches.values_of("targets").unwrap());
    let exclude = HashSet::from_iter(matches.values_of("exclude_directories").unwrap());
    let recurse = matches.is_present("recurse");
    let skip_confirmation = matches.is_present("skip_confirmation");
    let get_size = matches.is_present("get_size");

    Config {
        path,
        targets,
        exclude,
        recurse,
        skip_confirmation,
        get_size,
    }
}

fn scan() {
    let lib_path = PathBuf::from("/home/dv/partition/programming/scheme");
    let mut contents: Vec<String> = Vec::new();
    let mut dirs: Vec<PathBuf> = vec![lib_path];

    loop {
        if let Some(dir) = dirs.get(0) {
            let (mut local_contents, mut local_dirs) = get_dir_contents(dir);
            dirs.remove(0);
            contents.append(&mut local_contents);
            dirs.append(&mut local_dirs);
        } else {
            break;
        }
    }

    println!("{:#?}", contents);
}

fn get_dir_contents(path: &Path) -> (Vec<String>, Vec<PathBuf>) {
    let entries = path.read_dir().unwrap();
    let mut contents: Vec<String> = Vec::new();
    let mut dirs: Vec<PathBuf> = Vec::new();

    for entry in entries {
        let path = entry.unwrap().path();

        if path.is_file() {
            let file_name = path.file_name().unwrap();
            contents.push(file_name.to_str().unwrap().to_owned());
        } else if path.is_dir() {
            dirs.push(path);
        }
    }

    (contents, dirs)
}
